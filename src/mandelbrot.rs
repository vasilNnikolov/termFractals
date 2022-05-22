extern crate num;
use num::complex::Complex;
use std::sync::mpsc;
use std::thread;

use crate::term_io;
pub const IN_FRACTAL: char = '*';
pub const OUTSIDE_FRACTAL: char = ' ';

struct PixelWithCoords {
    coords: (u16, u16),
    value: term_io::Pixel
}

pub fn render_whole_mandelbrot(screen: &mut term_io::Screen) -> Result<(), &'static str> {
    let (w, h) = screen.term_size;
    let mut coords_to_draw: Vec<(Complex<f64>, (u16, u16))> = Vec::new();
    for x in 0..w {
        for y in 0..h {
            if let term_io::Pixel::Recompute = screen.buffer.get(x, y)? {
                coords_to_draw.push((screen.get_complex_coords(x, y)?, (x, y)));
            }
        }
    }

    let n_threads = 10;
    let chunk_size = (coords_to_draw.len()/n_threads) as usize;

    let mut bunches = Vec::new();
    let mut i = 0;
    while i < coords_to_draw.len() {
        let mut bunch = Vec::new();
        for j in 0..chunk_size {
            if i + j < coords_to_draw.len() {
                bunch.push(coords_to_draw[i+j]);
            } else { break; }
        }
        i += bunch.len();
        bunches.push(bunch);

    }

    let (tx, rx) = mpsc::channel::<PixelWithCoords>();
    let mut handles = Vec::new();

    for coord_bunch in bunches {
        let local_tx = tx.clone();
        let handle = thread::spawn(move || {
            for c in coord_bunch {
                local_tx.send(PixelWithCoords {
                    coords: c.1, 
                    value: term_io::Pixel::Value(if compute_mandelbrot_pixel(c.0, 100) {IN_FRACTAL} else {OUTSIDE_FRACTAL})
                }).unwrap();
            } 
        });
        handles.push(handle);
    }

    for h in handles {
        if let Err(_e) = h.join() {
            return Err("could not join one of the threads");
        }
    }

    for _i in 0..coords_to_draw.len() {
        let pixel = rx.recv();
        match pixel {
            Ok(px) => screen.putchar(px.coords.0, px.coords.1, px.value)?, 
            Err(_e) => { return Err("could not recieve from reciever"); }
        }
    }

    // screen.flush_screen()?;
    Ok(())
}

fn compute_mandelbrot_pixel(c: Complex<f64>, n_iter: u16) -> bool { // returns true if the pixel is in the set
    let mut in_set = true;
    let mut z = Complex::new(0.0, 0.0); 
    let mut z_norm;

    for _ in 0..n_iter {
        z = z*z + c; 
        z_norm = z.norm_sqr(); 
        if z_norm > 4.0 {
            in_set = false;
            break;
        }
        else if z_norm < 0.01 {
            break;
        }
    }
    return in_set;
}

fn render_mandelbrot_pixel(screen: &mut term_io::Screen, x: u16, y: u16) -> Result<(), &'static str> {
    if x >= screen.term_size.0 || y >= screen.term_size.1 {
        return Err("cannot render pixel outside of screen");
    }
    let mut in_set = true;
    let mut z = Complex::new(0.0, 0.0); 
    let c = screen.get_complex_coords(x, y)?;
    let mut z_norm;
    let n_iter = 100.0*(1.0 - 0.1*screen.scale.log10());

    for _i in 0..std::cmp::max(n_iter as i32, 500) {
        z = z*z + c; 
        z_norm = z.norm_sqr(); 
        if z_norm > 4.0 {
            in_set = false;
            break;
        }
        else if z_norm < 0.01 {
            break;
        }
    }
    if in_set {
        screen.putchar(x, y, term_io::Pixel::Value(IN_FRACTAL))?;
    } else {
        screen.putchar(x, y, term_io::Pixel::Value(OUTSIDE_FRACTAL))?;
    }
    Ok(())
}
