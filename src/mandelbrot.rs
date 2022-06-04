extern crate num;
use num::complex::Complex;
use std::sync::mpsc;
use std::thread;

use crate::term_io;
pub const IN_FRACTAL: char = '*';
pub const OUTSIDE_FRACTAL: char = ' ';
pub const MIN_ITER: i32 = 50;

struct PixelWithCoords {
    coords: (u16, u16),
    value: term_io::Pixel
}

fn get_iterations(scale: f64) -> u16 {
    return std::cmp::max(200*(1.0 - 0.8*scale.log10()) as i32, 1000) as u16;
}

pub fn render_whole_mandelbrot(screen: &mut term_io::Screen, n_iter: u16) -> Result<(), &'static str> {
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
    let mut j = 0;
    while i < coords_to_draw.len() {
        let mut bunch = Vec::new();
        while j + i < coords_to_draw.len() && j < chunk_size {
            bunch.push(coords_to_draw[j+i]);
            j += 1;
        }
        i += bunch.len();
        j = 0;
        bunches.push(bunch);
    }

    let (tx, rx) = mpsc::channel::<PixelWithCoords>();

    for coord_bunch in bunches {
        let local_tx = tx.clone();
        thread::spawn(move || {
            for c in coord_bunch {
                local_tx.send(PixelWithCoords {
                    coords: c.1, 
                    value: term_io::Pixel::Value(if compute_mandelbrot_pixel(c.0, n_iter) {IN_FRACTAL} else {OUTSIDE_FRACTAL})
                }).unwrap();
            } 
        });
    }

    for _i in 0..coords_to_draw.len() {
        let pixel = rx.recv();
        match pixel {
            Ok(px) => screen.putchar(px.coords.0, px.coords.1, px.value)?, 
            Err(_e) => { return Err("could not recieve from reciever"); }
        }
    }

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

