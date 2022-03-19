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

fn editor_refresh_screen(stdout: &mut RawTerminal<StdoutLock>) {
    write!(stdout, "{}", termion::clear::All).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1,1)).unwrap();
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
