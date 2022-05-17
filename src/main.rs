
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;

mod user_input;


struct Screen {
    stdin: termion::AsyncReader, 
    stdout: termion::raw::RawTerminal<std::io::Stdout>
}

fn setup_terminal() -> Screen {
    let stdout = stdout(); 
    let stdout = stdout.into_raw_mode().unwrap();
    let stdin = async_stdin();
    Screen {
        stdin: stdin, 
        stdout: stdout
    }
}

fn main() {
    let screen = setup_terminal();
    let mut stdin = screen.stdin;
    let mut stdout = screen.stdout;

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
        let c = user_input::get_char(&mut stdin);
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
