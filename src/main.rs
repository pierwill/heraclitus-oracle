use std::collections::BTreeMap;
use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const Q: &str = "press q to exit";

fn main() {
    // setup screen
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

    // initiate variables
    let mut all_keys: Vec<char> = vec![];
    let mut guesses = Guesses::default();
    let mut model = Model {
        map: BTreeMap::default(),
    };

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') | Key::Char('q') => break,
            Key::Char(k) => {
                // record key press
                match k {
                    k if k == 'f' => all_keys.push('f'),
                    k if k == 'd' => all_keys.push('d'),
                    _ => continue,
                };

                let lastfive: Vec<char> = all_keys.clone().into_iter().rev().take(5).collect();
                let predicted = predict(model.clone(), lastfive.clone());
                let observed = lastfive.iter().rev().last().unwrap();
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
                    predicted,
                    observed,
                    termion::cursor::Goto(1, 3),
                )
                .unwrap();

                // record guesses
                guesses.total += 1;
                if predicted == *observed {
                    guesses.correct += 1;
                }
                println!(
                    "guess %: {:.1}",
                    (guesses.correct as f32 / guesses.total as f32) * 100.0
                );

                // update model
                model = update_model(model, all_keys.clone());
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    // write data to file
    let mut out = String::new();
    out.push_str(format!("key history: {all_keys:?}\n").as_str());
    for entry in &model.map {
        out.push_str(format!("{:?}\n", entry).as_str());
    }
    out.push_str(format!("number of entries in map: {}\n", model.map.len()).as_str());
    std::fs::write("out", out.as_bytes()).expect("oops");
}

/// A map giving the score for each fivegram.
#[derive(Default, Debug, Clone)]
struct Model {
    map: BTreeMap<Vec<char>, Score>,
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
    total: i32,
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

fn update_model(mut m: Model, all_keys: Vec<char>) -> Model {
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

    let current_score: Option<&Score> = m.map.get(&lastfive);
    let mut new_score = current_score.unwrap_or(&Score::default()).clone();
    if last == 'f' {
        new_score.f += 1;
    } else if last == 'd' {
        new_score.d += 1;
    }
    m.map.insert(lastfive, new_score);

    m
}
