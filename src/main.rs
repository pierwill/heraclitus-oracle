#![allow(dead_code)]

use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const Q: &str = "press q to exit";

fn main() {
    // let mut stdin = termion::async_stdin().keys();
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        Q,
    )
    .unwrap();
    stdout.flush().unwrap();

    write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();

    let mut all_keys: Vec<char> = vec![];

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') | Key::Char('q') => break,
            Key::Char('f') => {
                all_keys.push('f');
                write!(stdout, "f{}", termion::cursor::Goto(1, 2)).unwrap()
            }
            Key::Char('d') => {
                all_keys.push('d');
                write!(stdout, "d{}", termion::cursor::Goto(1, 2)).unwrap()
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    println!("{all_keys:?}");
}

fn update_model_f() {
    // function updateModelF (fivegram) {
    //   return function (letter) {
    //     var fg = model[fivegram]
    //     if (!fg) {
    //       model[fivegram] = { f: 0, d: 0 }
    //     }
    //     model[fivegram][letter]+=1
    //     return
    //   }
    // }
}
fn predict_next_letter() {
    // function predictNextLetter (fivegram) {
    //   var m = model[fivegram]
    //   if (!m)
    //     return 'f'
    //   if (m.f > m.d)
    //     return 'f'
    //   return 'd'
    // }
}
fn predict(input: char) {
    // function predict (inputS) {
    //   var lastSix = inputS.slidingWindow(6,6)
    //   return lastSix.map(s => {
    //     var fiveGram = _.slice(s, 0,5).join('')
    //     // predict next value
    //     var prediction = predictNextLetter(fiveGram)
    //     //make a fn to update model after i see real value
    //     var updateModel = updateModelF(fiveGram)
    //     // get the next letter now
    //     var last = _.last(s)
    //     // update my model with it (HACK SIDE-EFFECTY)
    //     updateModel(last)
    //     return [prediction, last]
    //   })
    // }
}
