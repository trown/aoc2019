use crate::intcode::Program;

use std::fmt;

/// Parses each line to be an i64
#[aoc_generator(day9)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

struct OutputWrapper(Vec<i64>);

impl fmt::Display for OutputWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[aoc(day9, part1)]
fn part_one(input: &[i64]) -> OutputWrapper {
    let mut program = Program::new(input.to_vec(), Vec::new());
    let mut input = vec![1];
    program.run(&mut input);
    OutputWrapper(program.output)
}

#[aoc(day9, part2)]
fn part_two(input: &[i64]) -> OutputWrapper {
    let mut program = Program::new(input.to_vec(), Vec::new());
    let mut input = vec![2];
    program.run(&mut input);
    OutputWrapper(program.output)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day_9_part_1() {
        let mut program = Program::new(vec![104, 1125899906842624, 99], Vec::new());
        let mut input = Vec::new();
        program.run(&mut input);
        assert_eq!(program.output, [1125899906842624]);
        program = Program::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], Vec::new());
        program.run(&mut input);
        assert_eq!(program.output, [1219070632396864]);
        program = Program::new(
            vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            Vec::new(),
        );
        program.run(&mut input);
        assert_eq!(
            program.output,
            [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        )
    }
}
