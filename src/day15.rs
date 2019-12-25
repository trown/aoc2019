use crate::intcode::Program;

use std::collections::HashMap;
use std::fmt;

/// Parses each line to be an i64
#[aoc_generator(day15)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

struct Map(HashMap<(i32, i32), (i32, bool)>);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for i in (-20..25).rev() {
            for j in -25..15 {
                match self.0.get(&(j, i)) {
                    Some((0, _)) => write!(f, " # ")?,
                    Some((1, _)) => write!(f, " O ")?,
                    Some((2, _)) => write!(f, " * ")?,
                    Some((_, _)) => write!(f, " . ")?,
                    None => write!(f, " . ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn turn_left(d: i64) -> i64 {
    match d {
        1 => 3,
        2 => 4,
        3 => 2,
        4 => 1,
        _ => panic!("Invalid Direction"),
    }
}

fn turn_right(d: i64) -> i64 {
    match d {
        1 => 4,
        2 => 3,
        3 => 1,
        4 => 2,
        _ => panic!("Invalid Direction"),
    }
}

fn insert_wall(m: &mut Map, x: i32, y: i32, d: i64) {
    match d {
        1 => m.0.insert((x, y + 1), (0, false)),
        2 => m.0.insert((x, y - 1), (0, false)),
        3 => m.0.insert((x - 1, y), (0, false)),
        4 => m.0.insert((x + 1, y), (0, false)),
        _ => None,
    };
}

fn move_player(x: &mut i32, y: &mut i32, d: i64) {
    match d {
        1 => *y += 1,
        2 => *y -= 1,
        3 => *x -= 1,
        4 => *x += 1,
        _ => (),
    };
}

#[aoc(day15, part1)]
fn part_one(input: &[i64]) -> i32 {
    let mut program = Program::new(input.to_vec(), Vec::new());
    let mut map: Map = Map(HashMap::new());
    let mut moves = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut direction = 1;
    let mut i = 0;
    let mut optimal_moves = 0;
    map.0.insert((x, y), (1, true));
    moves.push(direction);
    //println!("{:?}", (x, y));
    loop {
        program.next(&mut moves);
        if let Some(status) = program.output.pop() {
            //println!("status: {}", status);
            match status {
                0 => {
                    //println!("{:?}", (x, y));
                    //println!("wall");
                    insert_wall(&mut map, x, y, direction);
                    direction = turn_left(direction);
                    moves.push(direction);
                }
                1 => {
                    //println!("{:?}", (x, y));
                    //println!("open");
                    if let Some(this) = map.0.get(&(x, y)) {
                        //println!("this: {:?}", this);
                        if this.1 {
                            map.0.insert((x, y), (3, true));
                        } else {
                            map.0.insert((x, y), (3, false));
                        }
                    } else {
                        map.0.insert((x, y), (3, true));
                    }
                    move_player(&mut x, &mut y, direction);
                    let next = map.0.get(&(x, y));
                    //println!("next: {:?}", next);
                    match next {
                        Some((_, true)) => {
                            direction = turn_right(direction);
                            moves.push(direction);
                            optimal_moves -= 1;
                            map.0.insert((x, y), (1, true));
                        }
                        Some((_, false)) => {
                            direction = turn_right(direction);
                            moves.push(direction);
                            optimal_moves += 1;
                            map.0.insert((x, y), (1, true));
                        }
                        None => {
                            moves.push(direction);
                            optimal_moves += 1;
                            map.0.insert((x, y), (1, true));
                        }
                    }
                }
                2 => {
                    println!("{:?}", (x, y));
                    println!("oxygen");
                    move_player(&mut x, &mut y, direction);
                    map.0.insert((x, y), (2, false));
                    direction = turn_right(direction);
                    direction = turn_right(direction);
                    moves.push(direction);
                    //println!("{}", map);
                    //break;
                }
                _ => panic!("Invalid Status"),
            }
            //println!("{}", map);
            //println!("{}", optimal_moves);
            if i > 30000 {
                break;
            } else {
                i += 1
            }
        }
    }
    println!("{}", map);
    let spaces: Vec<&(i32, i32)> = map
        .0
        .keys()
        .filter(|k| map.0.get(k).unwrap().0 == 1 || map.0.get(k).unwrap().0 == 3)
        .collect();
    println!("{:?}", spaces);
    optimal_moves
}
