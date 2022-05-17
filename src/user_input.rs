use termion;
use std::io::Read;

pub fn get_char(stdin: &mut termion::AsyncReader) -> Option<u8>{
    let mut char_buffer: Vec<u8> = Vec::new();
    if let Err(e) = stdin.read_to_end(&mut char_buffer) {
        panic!("Error reading to stdin {}", e)
    }

    match char_buffer.get(0) {
        Some(c) => Some(*c), 
        None => None
    }
}
