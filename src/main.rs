
use std::thread;
use std::time::Duration;

mod terminal;
use terminal::{
    screen, 
    async_input, 
    cyclic_buffer
};
use cyclic_buffer::Direction;
mod mandelbrot;
mod stat_bar;

fn main() {
    if let Err(e) = run() {
        println!("main exited with error: {}", e);
    }
}

fn run() -> Result<(), &'static str> {
    let mut screen = screen::Screen::new_screen();
    
    screen.clear_screen()?;

    let mut should_end_program = false;
    let zoom_in = 1.2;
    let zoom_out = 1.0/zoom_in;
    let move_speed = 2.0*screen.term_size.0 as f64 / 100.0;
    let move_speed = std::cmp::max(1, move_speed as u16); 
    let mut n_iter_additive: i32 = 0;
    loop {
        screen.clear_screen()?;
        // render the status bar
        stat_bar::clear_stat_bar(&mut screen)?;
        let n_iter: i32 = std::cmp::max((200.0 - 100.0*screen.scale.log10()) as i32 + n_iter_additive, mandelbrot::MIN_ITER) ;
        stat_bar::render_status_bar(&mut screen, 0, n_iter as u16)?;

        mandelbrot::render_whole_mandelbrot(&mut screen, n_iter as u16)?;
        screen.render()?;
        loop {
            let c = async_input::get_char(&mut screen);
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
                // iteration control
                Some('n') => {
                    n_iter_additive += 5; 
                    screen.buffer.clear(cyclic_buffer::Pixel::Recompute);
                    break;
                }, 
                Some('m') => {
                    if n_iter - 5 > mandelbrot::MIN_ITER {
                        n_iter_additive -= 5;
                        screen.buffer.clear(cyclic_buffer::Pixel::Recompute);
                    }
                    break;
                }, 
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
