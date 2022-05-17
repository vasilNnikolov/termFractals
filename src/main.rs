extern crate termion;

use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;


fn get_char(stdin: &mut termion::AsyncReader) -> Option<u8>{
    let mut char_buffer: Vec<u8> = Vec::new();
    if let Err(e) = stdin.read_to_end(&mut char_buffer) {
        panic!("Error reading to stdin {}", e)
    }

    match char_buffer.get(0) {
        Some(c) => Some(*c), 
        None => None
    }
}

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin();

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
            .unwrap();

    let mut height = 0;
    loop {
        write!(stdout, "{}{}", 
            termion::cursor::Goto(1, 1), 
            termion::clear::All)
            .unwrap();
        let c = get_char(&mut stdin);
        write!(stdout, "{:?} char entered {}", c, height).unwrap();
        write!(stdout, "{}height {}", termion::cursor::Goto(1, 5), height).unwrap();
        match c {
            Some(b'q') => break, 
            Some(b'j') => height -= 1, 
            Some(b'k') => height += 1, 
            _ => {}
        }

        thread::sleep(Duration::from_millis(50));
        
        stdout.flush().unwrap();
    }
}
