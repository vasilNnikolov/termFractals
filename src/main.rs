
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
    // let v0 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // let n = 3;
    // let chunk_size = (v0.len()/n) as usize;

    // let mut bunches = Vec::new();
    // let mut i = 0;
    // while i < v0.len() {
    //     let mut bunch = Vec::new();
    //     for j in 0..chunk_size {
    //         if i + j < v0.len() {
    //             bunch.push(v0[i+j]);
    //         } else { break; }
    //     }
    //     i += bunch.len();
    //     bunches.push(bunch);

    // }
    // println!("{:?}", bunches);
}

fn run() -> Result<(), &'static str>{
    let mut screen = term_io::setup_terminal();
    
    screen.clear_screen()?;

    let mut should_end_program = false;
    let zoom_in = 1.2;
    let zoom_out = 1.0/zoom_in;
    let move_speed = 2.0*screen.term_size.0 as f64 / 100.0;
    let move_speed = std::cmp::max(1, move_speed as u16); 
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
                Some('l') => {screen.on_move(Direction::Left, move_speed)?; break;}
                Some('k') => {screen.on_move(Direction::Down, move_speed)?; break;}
                Some('j') => {screen.on_move(Direction::Up, move_speed)?; break;}
                Some('h') => {screen.on_move(Direction::Right, move_speed)?; break;}
                // zoom control
                Some('x') => {screen.on_zoom(zoom_out)?; break;}
                Some('z') => {screen.on_zoom(zoom_in)?; break;}
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
