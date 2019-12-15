use crate::intcode::Program;

use std::collections::HashMap;
use std::fmt;

/// Parses each line to be an i64
#[aoc_generator(day13)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

struct Game(HashMap<(i32, i32), i32>);

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for i in 0..25 {
            for j in 0..40 {
                match self.0.get(&(j, i)) {
                    Some(1) => write!(f, " # ")?,
                    Some(2) => write!(f, " O ")?,
                    Some(3) => write!(f, "-_-")?,
                    Some(4) => write!(f, " * ")?,
                    Some(_) => write!(f, " . ")?,
                    None => write!(f, " . ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc(day13, part1)]
fn part_one(input: &[i64]) -> usize {
    let mut program = Program::new(input.to_vec(), Vec::new());
    program.run(&mut vec![0]);
    let mut game: Game = Game(HashMap::new());
    while program.output.len() > 2 {
        let square: Vec<i32> = program.output.drain(..3).map(|x| x as i32).collect();
        game.0.insert((square[0], square[1]), square[2]);
    }
    println!("{}", game);
    game.0.values().filter(|x| *x == &2).count()
}

#[aoc(day13, part2)]
fn part_two(input: &[i64]) -> i32 {
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
    loop {
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
                }
                (x, _, 4) => {
                    ball_x = x;
                    _block = game.0.insert((x, y), tile);
                }
                (x, _, 3) => {
                    paddle_x = x;
                    _block = game.0.insert((x, y), tile);
                }
                _ => {
                    _block = game.0.insert((x, y), tile);
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
    }
    score
}
