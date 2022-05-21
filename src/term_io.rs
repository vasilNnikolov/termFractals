extern crate num;
use num::complex::Complex;

use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, stdout};
use crate::cyclic_buffer::{Buffer, Direction};

pub struct Screen {
    pub stdin: termion::AsyncReader, 
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>, 
    pub term_size: (u16, u16), 
    pub scale: f64, 
    scale_change: f64, // how much the scale has changed since the last zoom
    pub center: Complex<f64>,
    pub buffer: Buffer<Option<char>>,
    vertical_scaling_constant: f64
}

pub fn setup_terminal() -> Screen {
    let stdout = stdout(); 
    let stdout = stdout.into_raw_mode().unwrap();
    let stdin = async_stdin();
    let (w, h) = termion::terminal_size().unwrap();
    let buffer: Buffer<Option<char>> = Buffer::new((w, h), None);

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
    pub fn putchar(&mut self, x: u16, y: u16, c: Option<char>) -> Result<(), &'static str>{
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
                if let None = c {
                    return Err("cannot render screen with None pixels");
                }
                let res = write!(self.stdout,
                       "{}{}",
                       termion::cursor::Goto(x + 1, y + 1), 
                       c.unwrap());
                if let Err(_e) = res {
                    return Err("could not write to screen during render");
                }
            }
        } 
        Ok(())
    }
    pub fn on_move(&mut self, direction: Direction) -> Result<(), &'static str>{
        self.buffer.shift(direction, 1, None)?;
        match direction {
            Direction::Right => {
                self.center += Complex::new(-self.scale, 0.0);
            },
            Direction::Left => {
                self.center += Complex::new(self.scale, 0.0);
            },
            Direction::Up => {
                self.center += Complex::new(0.0, -self.scale*self.vertical_scaling_constant);
            },
            Direction::Down => {
                self.center += Complex::new(0.0, self.scale*self.vertical_scaling_constant);
            },
        }
        Ok(())
    }
    pub fn on_zoom(&mut self, zoom_amount: f64) -> Result<(), &'static str> { // if zoom_amount > 1 => zoom in, else => zoom out
        self.scale_change /= zoom_amount;
        // check if with this scale change you have to modify the screen 
        if (1.0 - self.scale_change).abs()*(std::cmp::max(self.term_size.0, self.term_size.1) as f64) > 2.0 {
            // we need to make a change
            let (w, h) = self.term_size;
            let mut buff: Buffer<Option<char>> = Buffer::new((w, h), None); 
            buff.pointers = self.buffer.pointers;
            let allowed_error = 0.1;
            for x in 0..w {
                for y in 0..h {
                    let new_x = w as f64 / 2.0 + (x as f64 - w as f64 / 2.0) / self.scale_change;
                    let new_y = h as f64 / 2.0 + (y as f64 - h as f64 / 2.0) / self.scale_change; 
                    let rounded_x = new_x.round() as i32;
                    let rounded_y = new_y.round() as i32;
                    if 0 <= rounded_x && rounded_x < w as i32 && 0 <= rounded_y && rounded_y < h as i32 {
                        // let new_x = new_x as u16;
                        // let new_y = new_y as u16;
                        
                        let abs_difference_x: f64 = (new_x - rounded_x as f64).abs();
                        let abs_difference_y: f64 = (new_y - rounded_y as f64).abs();
                        if abs_difference_y < allowed_error && abs_difference_x < allowed_error {
                            buff.put(self.buffer.get(x, y)?, rounded_x as u16, rounded_y as u16)?;                
                        }
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



