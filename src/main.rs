
// use std::io::Write;
use std::thread;
use std::time::Duration;

mod user_input;
mod term_io;
mod cyclic_buffer;
mod mandelbrot;

use cyclic_buffer::Direction;

fn main() {
    if let Err(e) = run() {
        println!("main exited with error: {}", e);
    }
}

fn run() -> Result<(), &'static str>{
    let mut screen = term_io::setup_terminal();
    
    screen.clear_screen()?;

    let mut should_end_program = false;
    loop {
        screen.clear_screen()?;
        mandelbrot::render_whole_mandelbrot(&mut screen)?;
        screen.render()?;
        loop {
            let c = user_input::get_char(&mut screen);
            match c {
                None => continue,
                Some('q') => {should_end_program = true; break;}, 
                // movement controlls
                Some('h') => {screen.on_move(Direction::Left)?; break;}
                Some('j') => {screen.on_move(Direction::Down)?; break;}
                Some('k') => {screen.on_move(Direction::Up)?; break;}
                Some('l') => {screen.on_move(Direction::Right)?; break;}
                // zoom control
                // Some('z') => {screen.scale *= zoom_speed; break;}
                // Some('x') => {screen.scale /= zoom_speed; break;}
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
