#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod mean;

const Q: &str = "press q to exit";

fn main() {
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
    let mut model = Model {
        map: HashMap::default(),
    };

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') | Key::Char('q') => break,
            Key::Char('f') => {
                all_keys.push('f');
                let lastfive: Vec<char> = all_keys.clone().into_iter().rev().take(5).collect();
                write!(
                    stdout,
                    "{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)
                )
                .unwrap();
                write!(stdout, "{:?}{}", lastfive, termion::cursor::Goto(1, 2),).unwrap();
                write!(
                    stdout,
                    "predicted: {}, observed: {}{}",
                    predict(model.clone(), lastfive.clone()), // predicted
                    lastfive.clone().iter().rev().last().unwrap(), // observed
                    termion::cursor::Goto(1, 3),
                )
                .unwrap();
                model = update_model_f(model, all_keys.clone());
            }
            Key::Char('d') => {
                all_keys.push('d');
                let lastfive: Vec<char> = all_keys.clone().into_iter().rev().take(5).collect();
                write!(
                    stdout,
                    "{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)
                )
                .unwrap();
                write!(stdout, "{:?}{}", lastfive, termion::cursor::Goto(1, 2),).unwrap();
                write!(
                    stdout,
                    "predicted: {}, observed: {}{}",
                    predict(model.clone(), lastfive.clone()), // predicted
                    lastfive.clone().iter().rev().last().unwrap(), // observed
                    termion::cursor::Goto(1, 3),
                )
                .unwrap();
                model = update_model_f(model, all_keys.clone());
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    println!("{all_keys:?}");
}

/// A map like this
///
///   [
///   'ffffg': Score,
///    ...
///   ]
///
/// for each 5-gram.
#[derive(Debug, Clone)]
struct Model {
    map: HashMap<Vec<char>, Score>,
}

/// For each fivegram, f is the number
/// of times f was pressed next, and likewise
/// for d.
#[derive(Debug, Clone)]
struct Score {
    f: i32,
    d: i32,
}

fn update_model_f(m: Model, all_keys: Vec<char>) -> Model {
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
    let fivegram: Vec<char> = all_keys.into_iter().rev().take(5).collect();

    m
}

fn predict(m: Model, all_keys: Vec<char>) -> char {
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

    // function predictNextLetter (fivegram) {
    //   var m = model[fivegram]
    //   if (!m)
    //     return 'f'
    //   if (m.f > m.d)
    //     return 'f'
    //   return 'd'
    // }
    let fivegram: Vec<char> = all_keys.into_iter().rev().take(5).collect();

    'f'
}
