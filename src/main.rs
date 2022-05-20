
// use std::io::Write;
use std::thread;
use std::time::Duration;


mod user_input;
mod term_io;
mod cyclic_buffer;
mod mandelbrot;

fn main() {
    if let Err(e) = run() {
        println!("main exited with error: {}", e);
    }
}

// fn test_run() -> Result<(), &'static str> {
//     let mut screen = term_io::setup_terminal();
//     screen.clear_screen()?;
//     let mut buffer: cyclic_buffer::Buffer<Option<char>> = cyclic_buffer::Buffer::new(screen.term_size, None);
//     buffer.put(Some('a'), 0, 0)?;
//     buffer.put(Some('b'), 1, 1)?;
//     buffer.put(Some('j'), 2, 2)?;
//     buffer.put(Some('d'), 3, 3)?;
//     buffer.shift(cyclic_buffer::Direction::Down, 3, None)?;
//     buffer.shift(cyclic_buffer::Direction::Right, 3, None)?;
//     for y in 0..buffer.size.1 {
//         for x in 0..buffer.size.0 {
//             if let Some(c) = buffer.get(x, y)? {
//                 screen.putchar(x, y, c)?;
//             }
//         }
//     } 
//     screen.flush_screen()?;
//     thread::sleep(Duration::from_millis(2000));
//     Ok(())
// }
fn run() -> Result<(), &'static str>{
    let mut screen = term_io::setup_terminal();
    
    screen.clear_screen()?;

    let mut should_end_program = false;
    let zoom_speed = 1.01;
    let move_speed = 0.02;
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
                // Some('h') => {screen.center += Complex::new(move_speed, 0.0); break;}
                // Some('l') => {screen.center += Complex::new(-move_speed, 0.0); break;}
                // Some('j') => {screen.center += Complex::new(0.0, move_speed); break;}
                // Some('k') => {screen.center += Complex::new(0.0, -move_speed); break;}
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
