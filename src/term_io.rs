extern crate num;
use num::complex::Complex;

use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, stdout};

pub struct Screen {
    pub stdin: termion::AsyncReader, 
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>, 
    pub term_size: (u16, u16), 
    pub scale: f64, 
    pub center: Complex<f64>,
}

pub fn setup_terminal() -> Screen {
    let stdout = stdout(); 
    let stdout = stdout.into_raw_mode().unwrap();
    let stdin = async_stdin();
    let (w, h) = termion::terminal_size().unwrap();

    Screen {
        stdin: stdin, 
        stdout: stdout,
        term_size: (w, h),
        scale: 0.02, 
        center: Complex::new(0.0, 0.0),
    }
}

impl Screen {
    pub fn get_complex_coords(&self, x: u16, y: u16) -> Result<Complex<f64>, &'static str> {
        if x < self.term_size.0 && y < self.term_size.1 {
            let (w, h) = self.term_size;
            let x_c = ((x as f64) - (w as f64)/2.0)*self.scale;
            let y_c = ((y as f64) - (h as f64)/2.0)*self.scale*2.0;
            return Ok(self.center + Complex::new(x_c, y_c));
        }
        Err("specified screen coordinates not on screen")
    }
}

pub fn clear_screen(screen: &mut Screen) {
    write!(screen.stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
            .unwrap();
}

pub fn putchar(screen: &mut Screen, x: u16, y: u16, c: char) {
    write!(screen.stdout,
           "{}{}",
           termion::cursor::Goto(x, y), 
           c)
            .unwrap();
}
