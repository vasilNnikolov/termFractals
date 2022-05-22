use crate::term_io;
use std::io::Read;

pub fn get_char(screen: &mut term_io::Screen) -> Option<char>{
    let mut char_buffer: Vec<u8> = Vec::new();
    if let Err(_e) = screen.stdin.read_to_end(&mut char_buffer) {
        panic!("Error reading to stdin ")
    }

    match char_buffer.get(0) {
        Some(c) => Some(*c as char), 
        None => None
    }
}
