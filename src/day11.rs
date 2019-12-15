use crate::intcode::Program;

use std::collections::HashMap;
use std::fmt;

/// Parses each line to be an i64
#[aoc_generator(day11)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct HullPaintingRobot {
    facing: Direction,
    current_position: (u8, u8),
    squares_painted: HashMap<(u8, u8), u8>,
    program: Program,
}

impl HullPaintingRobot {
    fn new(program: Program) -> Self {
        let facing = Direction::Up;
        let current_position = (0, 10);
        let squares_painted = HashMap::new();
        HullPaintingRobot {
            facing,
            current_position,
            squares_painted,
            program,
        }
    }
    fn paint(&mut self, init: Vec<i64>) {
        let mut input = init.clone();
        while self.program.next(&mut input) {
            if self.program.output.len() > 1 {
                let d = self.program.output.pop().unwrap() as u8;
                let c = self.program.output.pop().unwrap() as u8;
                self.squares_painted.insert(self.current_position, c);
                self.turn(d);
                input.clear();
                input.push(
                    *self
                        .squares_painted
                        .get(&self.current_position)
                        .map_or_else(|| &0, |c| c) as i64,
                );
            }
        }
    }
    fn turn(&mut self, d: u8) {
        match d {
            0 => match &self.facing {
                Direction::Up => {
                    self.facing = Direction::Left;
                    self.current_position.0 -= 1
                }
                Direction::Down => {
                    self.facing = Direction::Right;
                    self.current_position.0 += 1
                }
                Direction::Left => {
                    self.facing = Direction::Down;
                    self.current_position.1 -= 1
                }
                Direction::Right => {
                    self.facing = Direction::Up;
                    self.current_position.1 += 1
                }
            },
            1 => match &self.facing {
                Direction::Up => {
                    self.facing = Direction::Right;
                    self.current_position.0 += 1
                }
                Direction::Down => {
                    self.facing = Direction::Left;
                    self.current_position.0 -= 1
                }
                Direction::Left => {
                    self.facing = Direction::Up;
                    self.current_position.1 += 1
                }
                Direction::Right => {
                    self.facing = Direction::Down;
                    self.current_position.1 -= 1
                }
            },
            _ => panic!("Invalid output for turn function"),
        }
    }
}

impl fmt::Display for HullPaintingRobot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for i in 0..41 {
            for j in 4..12 {
                match self.squares_painted.get(&(i, j)) {
                    Some(1) => write!(f, "#")?,
                    Some(_) => write!(f, ".")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc(day11, part1)]
fn part_one(input: &[i64]) -> usize {
    let program = Program::new(input.to_vec(), Vec::new());
    let mut robot = HullPaintingRobot::new(program);
    robot.paint(vec![0]);
    robot.squares_painted.len()
}

#[aoc(day11, part2)]
fn part_two(input: &[i64]) -> HullPaintingRobot {
    let program = Program::new(input.to_vec(), Vec::new());
    let mut robot = HullPaintingRobot::new(program);
    robot.paint(vec![1]);
    robot
}
