#![allow(dead_code)]

use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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
    let mut guesses = Guess::default();

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
                model = update_model(model, all_keys.clone());
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
                model = update_model(model, all_keys.clone());
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    // write data to file
    let mut out = String::new();
    out.push_str(format!("key history: {all_keys:?}").as_str());
    out.push_str(format!("model: {:#?}", model.map).as_str());
    out.push_str(format!("number of entries in map: {}", model.map.len()).as_str());
    std::fs::write("out", out.as_bytes()).expect("oops");
}

/// A map giving the score for each fivegram.
#[derive(Default, Debug, Clone)]
struct Model {
    map: HashMap<Vec<char>, Score>,
}

/// For each fivegram, f is the number
/// of times f was pressed next, and likewise
/// for d.
#[derive(Default, Debug, Clone)]
struct Score {
    f: i32,
    d: i32,
}

#[derive(Default, Debug, Clone)]
struct Guesses {
    correct: i32,
    incorrect: i32,
}

fn update_model(m: Model, all_keys: Vec<char>) -> Model {
    let mut model = m;

    let last: char = all_keys
        .clone()
        .into_iter()
        .rev()
        .take(1)
        .collect::<Vec<_>>()[0];
    let lastfive: Vec<char> = all_keys.into_iter().rev().take(5).collect();

    if lastfive.len() < 5 {
        return Model::default();
    }

    let current_score: Option<&Score> = model.map.get(&lastfive);
    let mut new_score = current_score.unwrap_or(&Score::default()).clone();
    if last == 'f' {
        new_score.f += 1;
    } else if last == 'd' {
        new_score.d += 1;
    }
    model.map.insert(lastfive, new_score);

    model
}

fn predict(m: Model, all_keys: Vec<char>) -> char {
    let fivegram: Vec<char> = all_keys.into_iter().rev().take(5).collect();
    if m.map.is_empty() {
        return 'f';
    }
    let current_score = m.map.get(&fivegram);

    match current_score {
        Some(s) if s.f > s.d => 'f',
        Some(s) if s.d > s.f => 'd',
        // If we have no score, yet, predict 'f'.
        None => 'f',
        _ => unreachable!(),
    }
}

fn is_correct(predicted: char, got: char) -> bool {
    predicted == got
}
