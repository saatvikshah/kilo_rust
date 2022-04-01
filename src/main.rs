use std::{io::{self, Write, StdinLock, StdoutLock}};
use termion::{raw::{IntoRawMode, RawTerminal}, input::TermRead, event::Key};

#[derive(PartialEq)]
enum EventLoopState{
    Running,
    Terminate
}

struct CursorState{
    x: u16,
    y: u16,
    x_max: u16,
    y_max: u16
}

impl CursorState{
    pub fn new(x_max: u16, y_max: u16) -> CursorState {
        CursorState{
            x: 0,
            y: 0,
            x_max: x_max - 1,
            y_max: y_max - 1
        }
    }

    pub fn add_x(self: &mut CursorState, x_inc: u16) {
        let x_new = self.x.saturating_add(x_inc);
        self.x = x_new.min(self.x_max);
    }

    pub fn sub_x(self: &mut CursorState, x_dec: u16) {
        self.x = self.x.saturating_sub(x_dec);
    }

    pub fn add_y(self: &mut CursorState, y_inc: u16) {
        let y_new = self.y.saturating_add(y_inc);
        self.y = y_new.min(self.y_max);
    }

    pub fn sub_y(self: &mut CursorState, y_dec: u16) {
        self.y = self.y.saturating_sub(y_dec);
    }

    pub fn top(self: &mut CursorState) {
        self.y = 0;
    }

    pub fn bottom(self: &mut CursorState) {
        self.y = self.y_max;
    }

    pub fn left(self: &mut CursorState) {
        self.x = 0;
    }

    pub fn right(self: &mut CursorState) {
        self.x = self.x_max;
    }

    pub fn display_x(self: &CursorState) -> u16 {
        self.x + 1
    }

    pub fn display_y(self: &CursorState) -> u16 {
        self.y + 1
    }

    pub fn display_xmax(self: &CursorState) -> u16 {
        self.x_max + 1
    }

    pub fn display_ymax(self: &CursorState) -> u16 {
        self.y_max + 1
    }

    pub fn at_bottom(self: &CursorState) -> bool {
        self.y == self.y_max
    }
}

fn editor_process_keypress(stdin: &mut StdinLock, cursor: &mut CursorState) -> EventLoopState {
    let in_key = stdin.keys().next().unwrap().unwrap();
    match in_key {
        Key::Ctrl('w') => EventLoopState::Terminate,
        Key::Up => {
            cursor.sub_y(1);
            EventLoopState::Running
        },
        Key::Down => {
            cursor.add_y(1);
            EventLoopState::Running
        },
        Key::Left => {
            cursor.sub_x(1);
            EventLoopState::Running
        }
        Key::Right => {
            cursor.add_x(1);
            EventLoopState::Running
        }
        Key::PageUp => {
            cursor.top();
            EventLoopState::Running
        }
        Key::PageDown => {
            cursor.bottom();
            EventLoopState::Running
        },
        Key::Home => {
            cursor.left();
            EventLoopState::Running
        }
        Key::End => {
            cursor.right();
            EventLoopState::Running
        }
        _ => EventLoopState::Running
    }
}

fn editor_draw_rows(stdout: &mut RawTerminal<StdoutLock>, cursor: &CursorState) {
    write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    for _ in 0..(cursor.display_ymax() - 1) {
        write!(stdout, "{}~\r\n", termion::clear::CurrentLine).unwrap();
    }
    write!(stdout, "{}~", termion::clear::CurrentLine).unwrap();

    if cursor.at_bottom() {
       return; 
    }

    
    // TODO: StringBuilder like construct to create footer string 
    let footer_message = format!("kilo-rust -- [{}/{}, {}/{}]", cursor.display_x(), cursor.display_xmax(), cursor.display_y(), cursor.display_ymax());
    let footer_start_x = (usize::from(cursor.display_xmax()) - footer_message.len()) / 2;
    for _ in 1..footer_start_x {
        write!(stdout, " ").unwrap();
    }
    write!(stdout, "{}", footer_message).unwrap();
}

fn editor_refresh_screen(stdout: &mut RawTerminal<StdoutLock>, cursor: &mut CursorState) {
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    editor_draw_rows(stdout, cursor);
    write!(stdout, "{}", termion::cursor::Goto(cursor.display_x(), cursor.display_y())).unwrap();
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
    let (xlen, ylen) = termion::terminal_size().unwrap();
    let mut cursor = CursorState::new(xlen, ylen);
    while event_loop_state == EventLoopState::Running {
        editor_refresh_screen(&mut stdout, &mut cursor);
        event_loop_state = editor_process_keypress(&mut stdin, &mut cursor);
    }
    editor_refresh_screen(&mut stdout, &mut cursor);
}
