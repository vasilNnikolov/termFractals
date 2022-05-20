
// use std::io::Write;
use std::thread;
use std::time::Duration;

extern crate num;
use num::complex::Complex;

mod user_input;
mod term_io;
mod cyclic_buffer;

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
                screen.putchar(x, y, '#')?;
            }
        }
    } 
    screen.flush_screen()?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        println!("main exited with error: {}", e);
    }
}

fn test_run() -> Result<(), &'static str> {
    let mut screen = term_io::setup_terminal();
    screen.clear_screen()?;
    let mut buffer: cyclic_buffer::Buffer<Option<char>> = cyclic_buffer::Buffer::new(screen.term_size, None);
    buffer.put(Some('a'), 0, 0)?;
    buffer.put(Some('b'), 1, 1)?;
    buffer.put(Some('j'), 2, 2)?;
    buffer.put(Some('d'), 3, 3)?;
    buffer.shift(cyclic_buffer::Direction::Down, 3, None)?;
    buffer.shift(cyclic_buffer::Direction::Right, 3, None)?;
    for y in 0..buffer.size.1 {
        for x in 0..buffer.size.0 {
            if let Some(c) = buffer.get(x, y)? {
                screen.putchar(x, y, c)?;
            }
        }
    } 
    screen.flush_screen()?;
    thread::sleep(Duration::from_millis(2000));
    Ok(())
}
fn run() -> Result<(), &'static str>{
    let mut screen = term_io::setup_terminal();
    
    screen.clear_screen()?;

    let mut should_end_program = false;
    let zoom_speed = 1.01;
    let move_speed = 0.02;
    loop {
        screen.clear_screen()?;
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
        
        screen.flush_screen()?;
    }
    screen.clear_screen()?;

    Ok(())
}
