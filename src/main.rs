use std::io::{self, Write};
use termion::{raw::IntoRawMode, input::TermRead, event::Key};

fn main() {
    // Main program
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut keys = stdin.keys();

    loop{
        let in_key = keys.next().unwrap().unwrap();
        match in_key {
            Key::Char('q') => return,
            Key::Char(c) => {
                if c.is_control() {
                    // 'Enter'/'Ctrl-M': Fix carriage return ('\r\n' => '\n')
                    writeln!(stdout)
                } else {
                    write!(stdout, "{}", c)
                }
            },
            other => write!(stdout, "{:?}", other)
        }.unwrap();
        stdout.flush().unwrap();
    }
}
