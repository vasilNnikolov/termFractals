extern crate num;
use num::complex::Complex;

use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, stdout};
use crate::cyclic_buffer::{Buffer, Direction};
use crate::mandelbrot;

fn in_range<T>(x: T, lower: T, upper: T) -> bool where 
T: PartialOrd 
{
    return lower <= x && x < upper;
}
#[derive(Copy, Clone)]
pub enum Pixel where 
{
    Recompute, // a render value means we have to re-compute the pixel
    Value(char) // means we have a correct value in the buffer, no need to re-compute it
}

pub struct Screen {
    pub stdin: termion::AsyncReader, 
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>, 
    pub term_size: (u16, u16), 
    pub scale: f64, 
    scale_change: f64, // how much the scale has changed since the last zoom
    pub center: Complex<f64>,
    pub buffer: Buffer<Pixel>,
    vertical_scaling_constant: f64
}

pub fn setup_terminal() -> Screen {
    let stdout = stdout(); 
    let stdout = stdout.into_raw_mode().unwrap();
    let stdin = async_stdin();
    let (w, h) = termion::terminal_size().unwrap();
    let buffer: Buffer<Pixel> = Buffer::new((w, h), Pixel::Recompute);

    Screen {
        stdin: stdin, 
        stdout: stdout,
        term_size: (w, h),
        scale: 0.02, 
        scale_change: 1.0,
        center: Complex::new(0.0, 0.0),
        buffer: buffer, 
        vertical_scaling_constant: 2.0
    }
}

impl Screen {
    pub fn get_complex_coords(&self, x: u16, y: u16) -> Result<Complex<f64>, &'static str> {
        if x < self.term_size.0 && y < self.term_size.1 {
            let (w, h) = self.term_size;
            let x_c = ((x as f64) - (w as f64)/2.0)*self.scale;
            let y_c = -((y as f64) - (h as f64)/2.0)*self.scale*self.vertical_scaling_constant;
            return Ok(self.center + Complex::new(x_c, y_c));
        }
        Err("specified screen coordinates not on screen")
    }
    pub fn clear_screen(&mut self) -> Result<(), &'static str> {
        let res = write!(self.stdout,
               "{}{}",
               termion::clear::All, 
               termion::cursor::Goto(1, 1)); 
        if let Err(_e) = res {
            return Err("could not clear screen");
        }
        Ok(())
    }
    pub fn putchar(&mut self, x: u16, y: u16, c: Pixel) -> Result<(), &'static str>{
        self.buffer.put(c, x, y)?;
        Ok(())
    }
    pub fn flush_screen(&mut self) -> Result<(), &'static str> {
        if let Err(e) = self.stdout.flush() {
            return Err("could not flush screen");
        }
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), &'static str> {
        for x in 0..self.term_size.0 {
            for y in 0..self.term_size.1 {
                let c = self.buffer.get(x, y)?;
                match self.buffer.get(x, y)? {
                    Pixel::Recompute => {return Err("cannot render screen where some pixels are not computed");},
                    Pixel::Value(c) => {
                        let res = write!(self.stdout,
                               "{}{}",
                               termion::cursor::Goto(x + 1, y + 1), 
                               c);
                        if let Err(_e) = res {
                            return Err("could not write to screen during render");
                        }
                    }
                }
            }
        } 
        Ok(())
    }
    pub fn on_move(&mut self, direction: Direction, times: u16) -> Result<(), &'static str>{
        self.buffer.shift(direction, times, Pixel::Recompute)?;
        match direction {
            Direction::Right => {
                self.center += times as f64*Complex::new(-self.scale, 0.0);
            },
            Direction::Left => {
                self.center += times as f64*Complex::new(self.scale, 0.0);
            },
            Direction::Up => {
                self.center += times as f64*Complex::new(0.0, -self.scale*self.vertical_scaling_constant);
            },
            Direction::Down => {
                self.center += times as f64*Complex::new(0.0, self.scale*self.vertical_scaling_constant);
            },
        }
        Ok(())
    }
    pub fn on_zoom(&mut self, zoom_amount: f64) -> Result<(), &'static str> { // if zoom_amount > 1 => zoom in, else => zoom out
        self.scale_change /= zoom_amount;
        // check if with this scale change you have to modify the screen 
        if (1.0 - self.scale_change).abs()*(std::cmp::max(self.term_size.0, self.term_size.1) as f64) > 2.0 {
            let (w, h) = self.term_size;
            let mut buff: Buffer<Pixel> = Buffer::new((w, h), Pixel::Recompute); 
            buff.pointers = self.buffer.pointers;
            let neighborhood: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, 1), (0, -1), (1, -1), (1, 0), (1, 1)]; 

            for x in 0..w {
                for y in 0..h {
                    let old_x = w as f64 / 2.0 + (x as f64 - w as f64 / 2.0) * self.scale_change;
                    let old_y = h as f64 / 2.0 + (y as f64 - h as f64 / 2.0) * self.scale_change; 
                    let old_x = old_x.round() as i32;
                    let old_y = old_y.round() as i32;

                    if (!in_range(old_x, 0, w as i32)) || (!in_range(old_y, 0, h as i32)) { continue; }

                    let mut surely_in_fractal = true;
                    let mut surely_outside_fractal = true;
                    for cell in neighborhood.iter() {
                        let coords = (cell.0 + old_x, cell.1 + old_y);
                        if in_range(coords.0, 0, w as i32) && in_range(coords.1, 0, h as i32) {
                            match self.buffer.get(coords.0 as u16, coords.1 as u16)? {
                                Pixel::Value(mandelbrot::IN_FRACTAL) => { // cannot be sure it is outside of fractal
                                    surely_outside_fractal = false;
                                },
                                Pixel::Value(mandelbrot::OUTSIDE_FRACTAL) => {
                                    surely_in_fractal = false;
                                }, 
                                _ => {return Err("there was an unrendered pixel on the screen somehow");}
                            }
                            if (!surely_in_fractal) && (!surely_outside_fractal) {
                                break;
                            }
                        }
                    }
                    if surely_outside_fractal {
                        buff.put(Pixel::Value(mandelbrot::OUTSIDE_FRACTAL), x, y)?;
                    } else if surely_in_fractal {
                        buff.put(Pixel::Value(mandelbrot::IN_FRACTAL), x, y)?;
                    }
                }
            } 
            self.buffer = buff;
            self.scale *= self.scale_change;
            self.scale_change = 1.0;
        }
        Ok(()) 
    }
}



