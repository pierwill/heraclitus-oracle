#![allow(unused_imports)]
#![allow(dead_code)]

use std::io::{stdin, stdout, Write};

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    write!(
        stdout,
        "{}{}q to exit",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('f')) => write!(stdout, "{}f", termion::clear::All,).unwrap(),
            Event::Key(Key::Char('d')) => write!(stdout, "{}d", termion::clear::All,).unwrap(),
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
