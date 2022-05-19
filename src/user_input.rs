use crate::term_io;
use std::io::Read;

pub fn get_char(screen: &mut term_io::Screen) -> Option<u8>{
    let mut char_buffer: Vec<u8> = Vec::new();
    if let Err(e) = screen.stdin.read_to_end(&mut char_buffer) {
        panic!("Error reading to stdin {}", e)
    }

    match char_buffer.get(0) {
        Some(c) => Some(*c), 
        None => None
    }
}
