
use std::io::Write;
use std::thread;
use std::time::Duration;

extern crate num;
use num::complex::Complex;

mod user_input;
mod term_io;

fn render_mandlebrot(screen: &mut term_io::Screen) {
    let (w, h) = screen.term_size;
    let N = 100;
    for x in 0..w {
        for y in 0..h {
            let mut in_set = true;
            let mut z = Complex::new(0.0, 0.0); 
            let c = screen.get_complex_coords(x, y).unwrap();
            for i in 0..N {
                z = z*z + c; 
                if z.norm_sqr() > 4.0 {
                    in_set = false;
                    break;
                }
            }

            if in_set {
                term_io::putchar(screen, x, y, '#');
            }
        }
    } 
    screen.stdout.flush().unwrap();
}

fn main() {
    let mut screen = term_io::setup_terminal();
    
    
    term_io::clear_screen(&mut screen);

    let mut should_end_program = false;
    loop {
        term_io::clear_screen(&mut screen);
        render_mandlebrot(&mut screen);
        loop {
            let c = user_input::get_char(&mut screen);
            match c {
                None => continue,
                Some('q') => {should_end_program = true; break;}, 
                _ => {}
            }
        }
        if should_end_program { break; }

        thread::sleep(Duration::from_millis(50));
        
        screen.stdout.flush().unwrap();
    }
}
