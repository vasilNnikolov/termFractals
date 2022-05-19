
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, stdout};

pub struct Screen {
    pub stdin: termion::AsyncReader, 
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>
}

pub fn setup_terminal() -> Screen {
    let stdout = stdout(); 
    let stdout = stdout.into_raw_mode().unwrap();
    let stdin = async_stdin();
    Screen {
        stdin: stdin, 
        stdout: stdout
    }
}

pub fn clear_screen(screen: &mut Screen) {
    write!(screen.stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
            .unwrap();
}

// pub fn write(screen: &Screen, w: u32, h: u32) {
//     write! 
// }
