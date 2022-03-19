use std::{io::{self, Write, StdinLock, StdoutLock}};
use termion::{raw::{IntoRawMode, RawTerminal}, input::TermRead, event::Key};

#[derive(PartialEq)]
enum EventLoopState{
    Running,
    Terminate
}

fn editor_process_keypress(stdin: &mut StdinLock) -> EventLoopState {
    let in_key = stdin.keys().next().unwrap().unwrap();
    match in_key {
        Key::Ctrl('w') => EventLoopState::Terminate,
        _ => EventLoopState::Running
    }
}

fn editor_draw_rows(stdout: &mut RawTerminal<StdoutLock>) {
    let (xlen, ylen) = termion::terminal_size().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1,1)).unwrap();
    for _ in 0..(ylen - 1) {
        write!(stdout, "{}~\r\n", termion::clear::CurrentLine).unwrap();
    }
    write!(stdout, "{}~", termion::clear::CurrentLine).unwrap();
    // TODO: StringBuilder like construct to create footer string 
    const FOOTER_MSG: &str = "[kilo-rust -- version 0.0]";
    let footer_start_x = (usize::from(xlen) - FOOTER_MSG.len()) / 2;
    for _ in 1..footer_start_x {
        write!(stdout, " ").unwrap();
    }
    write!(stdout, "{}", FOOTER_MSG).unwrap();
}

fn editor_refresh_screen(stdout: &mut RawTerminal<StdoutLock>) {
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    editor_draw_rows(stdout);
    write!(stdout, "{}", termion::cursor::Goto(1,1)).unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();
}

fn main() {
    // Main program
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut event_loop_state = EventLoopState::Running;
    while event_loop_state == EventLoopState::Running {
        editor_refresh_screen(&mut stdout);
        event_loop_state = editor_process_keypress(&mut stdin);
    }
    editor_refresh_screen(&mut stdout);
}
