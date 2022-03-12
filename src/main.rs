use std::io::{self, Read, Write};
use termion::raw::IntoRawMode;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut bytes = stdin.bytes();

    loop{
        let user_char = bytes.next().unwrap().unwrap();
        match user_char {
            b'q' => return,
            c => write!(stdout, "{}", c as char)
        }.unwrap();
        stdout.flush().unwrap();
    }
}
