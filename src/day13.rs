use crate::intcode::Program;

use termion::async_stdin;
use termion::raw::IntoRawMode;
use termion::{color, cursor, style};

use std::collections::HashMap;
//use std::fmt;
use std::io::{stdout, Read, Write};
use std::thread;
use std::time;

/// Parses each line to be an i64
#[aoc_generator(day13)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}
struct Game(HashMap<(i32, i32), i32>);

#[aoc(day13, part1)]
fn part_one(input: &[i64]) -> usize {
    let mut program = Program::new(input.to_vec(), Vec::new());
    program.run(&mut vec![0]);
    let mut game: Game = Game(HashMap::new());
    while program.output.len() > 2 {
        let square: Vec<i32> = program.output.drain(..3).map(|x| x as i32).collect();
        game.0.insert((square[0], square[1]), square[2]);
    }
    game.0.values().filter(|x| *x == &2).count()
}

#[aoc(day13, part2)]
fn part_two(input: &[i64]) -> i32 {
    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = async_stdin();
    //let stdin = stdin.lock();
    let mut bytes = stdin.bytes();

    let mut free_game = input.to_vec();
    free_game[0] = 2;
    let mut moves = Vec::new();
    let mut program = Program::new(free_game, Vec::new());
    let mut game: Game = Game(HashMap::new());
    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut i = 0;
    let mut j = 0;
    let mut initialized = false;
    let mut sleep = 0;
    let mut speed = 2;

    write!(
        stdout,
        "{}{}{}{}{}AoC Arcade                  Score:      ",
        termion::clear::All,
        termion::cursor::Hide,
        color::Bg(color::Black),
        color::Fg(color::Red),
        termion::cursor::Goto(20, 9)
    );
    stdout.flush().unwrap();

    loop {
        stdout.flush().unwrap();
        i += 1;
        program.next(&mut moves);
        if program.output.len() > 2 {
            let (x, y, tile) = (
                program.output[0] as i32,
                program.output[1] as i32,
                program.output[2] as i32,
            );
            program.output.clear();
            let mut _block = std::option::Option::None;
            match (x, y, tile) {
                (-1, 0, tile) => {
                    score = tile;
                    write!(
                        stdout,
                        "{}{}{}{}",
                        color::Bg(color::Black),
                        color::Fg(color::Red),
                        termion::cursor::Goto(55, 9),
                        score
                    );
                }
                (x, y, 4) => {
                    ball_x = x;
                    _block = game.0.insert((x, y), tile);
                    write!(
                        stdout,
                        "{}{}{}*",
                        style::Reset,
                        color::Fg(color::Blue),
                        cursor::Goto(x as u16 + 20, y as u16 + 10)
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                }
                (x, y, 3) => {
                    paddle_x = x;
                    _block = game.0.insert((x, y), tile);
                    write!(
                        stdout,
                        "{}{}{}_",
                        style::Reset,
                        color::Fg(color::Yellow),
                        cursor::Goto(x as u16 + 20, y as u16 + 10)
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                }
                (x, y, 2) => {
                    _block = game.0.insert((x, y), tile);
                    write!(stdout, "{}O", cursor::Goto(x as u16 + 20, y as u16 + 10)).unwrap();
                    stdout.flush().unwrap();
                }
                _ => {
                    _block = game.0.insert((x, y), tile);
                    write!(
                        stdout,
                        "{}{}{}.",
                        style::Reset,
                        color::Fg(color::Green),
                        cursor::Goto(x as u16 + 20, y as u16 + 10)
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                }
            }
        }
        if paddle_x < ball_x {
            moves.push(1)
        } else if paddle_x > ball_x {
            moves.push(-1)
        } else {
            moves.push(0)
        }
        if i > 18000 && game.0.values().filter(|x| *x == &2).count() == 0 {
            j += 1;
        }
        if j > 100 {
            break;
        }
        if i > 18000 && !initialized {
            for y in 0..25 {
                for x in 0..40 {
                    match game.0.get(&(x, y)) {
                        Some(1) => write!(
                            stdout,
                            "{}{}{}#",
                            cursor::Goto(x as u16 + 20, y as u16 + 10),
                            color::Bg(color::Red),
                            color::Fg(color::Black)
                        )
                        .unwrap(),
                        Some(2) => write!(stdout, "{}{}O", style::Reset, color::Fg(color::Magenta))
                            .unwrap(),
                        Some(3) => {
                            write!(stdout, "{}{}_", style::Reset, color::Fg(color::Red)).unwrap()
                        }
                        Some(4) => {
                            write!(stdout, "{}{}*", style::Reset, color::Fg(color::Yellow)).unwrap()
                        }
                        Some(_) => write!(
                            stdout,
                            "{}{}{}.",
                            cursor::Goto(x as u16 + 20, y as u16 + 10),
                            style::Reset,
                            color::Fg(color::Green)
                        )
                        .unwrap(),
                        None => write!(
                            stdout,
                            "{}{}{}.",
                            cursor::Goto(x as u16 + 20, y as u16 + 10),
                            style::Reset,
                            color::Fg(color::Green)
                        )
                        .unwrap(),
                    }
                }
            }
            stdout.flush().unwrap();
            initialized = true;
        }
        if i > 18000 && sleep > speed {
            thread::sleep(time::Duration::from_millis(1));
            sleep = 0;
        } else {
            sleep += 1;
        }

        //write!(stdout, "{}{}", termion::clear::All, game);
        let b = bytes.next();
        match b {
            // Quit
            Some(Ok(b'q')) => break,
            Some(Ok(b' ')) => speed = speed * 10,
            // Clear the screen
            Some(Ok(b'c')) => write!(stdout, "{}", termion::clear::All).unwrap(),
            _ => continue,
        }
    }
    write!(stdout, "{}{}", style::Reset, cursor::Goto(1, 45)).unwrap();
    score
}
