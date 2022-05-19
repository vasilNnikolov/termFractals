
use std::io::Write;
use std::thread;
use std::time::Duration;

extern crate num;
use num::complex::Complex;

mod user_input;
mod term_io;

fn render_mandlebrot(screen: &mut term_io::Screen) -> Result<(), &'static str> {
    let (w, h) = screen.term_size;
    let N = 100;
    let mut z_norm;
    for x in 0..w {
        for y in 0..h {
            let mut in_set = true;
            let mut z = Complex::new(0.0, 0.0); 
            let c = screen.get_complex_coords(x, y)?;
            for i in 0..N {
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
                term_io::putchar(screen, x, y, '#')?;
            }
        }
    } 
    term_io::flush_screen(screen)?;

    Ok(())

}
fn main() {
    if let Err(e) = run() {
        println!("main exited with error: {}", e);
    }
}

fn run() -> Result<(), &'static str>{
    let mut screen = term_io::setup_terminal();
    
    term_io::clear_screen(&mut screen)?;

    let mut should_end_program = false;
    let move_speed = 0.015;
    let zoom_speed = 1.01;
    loop {
        term_io::clear_screen(&mut screen)?;
        render_mandlebrot(&mut screen)?;
        loop {
            let c = user_input::get_char(&mut screen);
            match c {
                None => continue,
                Some('q') => {should_end_program = true; break;}, 
                // movement controlls
                Some('h') => {screen.center += Complex::new(move_speed, 0.0); break;}
                Some('l') => {screen.center += Complex::new(-move_speed, 0.0); break;}
                Some('j') => {screen.center += Complex::new(0.0, move_speed); break;}
                Some('k') => {screen.center += Complex::new(0.0, -move_speed); break;}
                // zoom control
                Some('z') => {screen.scale *= zoom_speed; break;}
                Some('x') => {screen.scale /= zoom_speed; break;}
                _ => {}
            }
        }
        if should_end_program {
            break;
        }

        thread::sleep(Duration::from_millis(50));
        
        term_io::flush_screen(&mut screen)?;
    }
    term_io::clear_screen(&mut screen)?;

    Ok(())
}
