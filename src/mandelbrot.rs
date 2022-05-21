extern crate num;
use num::complex::Complex;

use crate::term_io;

pub fn render_whole_mandelbrot(screen: &mut term_io::Screen) -> Result<(), &'static str> {
    let (w, h) = screen.term_size;
    for x in 0..w {
        for y in 0..h {
            if let None = screen.buffer.get(x, y)? {
                render_mandelbrot_pixel(screen, x, y)?;
            } 
        }
    } 
    screen.flush_screen()?;
    Ok(())
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

    for i in 0..std::cmp::max(n_iter as i32, 500) {
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
        screen.putchar(x, y, Some('*'))?;
    } else {
        screen.putchar(x, y, Some(' '))?;
    }
    Ok(())
}
