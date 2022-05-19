
use std::io::Write;
use std::thread;
use std::time::Duration;

mod user_input;
mod term_io;


fn main() {
    let mut screen = term_io::setup_terminal();
    
    term_io::clear_screen(&mut screen);

    let mut height = 0;
    loop {
        term_io::clear_screen(&mut screen);
        let c = user_input::get_char(&mut screen);
        write!(screen.stdout, "{:?} char entered {}", c, height).unwrap();
        write!(screen.stdout, "{}height {}", termion::cursor::Goto(1, 5), height).unwrap();
        match c {
            Some(b'q') => break, 
            Some(b'j') => height -= 1, 
            Some(b'k') => height += 1, 
            _ => {}
        }

        thread::sleep(Duration::from_millis(50));
        
        screen.stdout.flush().unwrap();
    }
}
