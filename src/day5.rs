/// Parses each line to be an i32
#[aoc_generator(day5)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

#[derive(Clone, Debug)]
pub struct Program {
    /// Data of the program (parsed input)
    data: Vec<i32>,
    /// Input to program
    input: i32,
    /// Output from program
    output: i32,
    /// Code pointer
    pointer: usize,
}

impl Program {
    pub fn new(data: Vec<i32>, input: i32) -> Self {
        Program {
            data,
            input,
            output: 0,
            pointer: 0,
        }
    }

    pub fn run_diagnostics(&mut self) -> i32 {
        while self.next() {}
        self.output
    }

    // Continues the execution of the program, returning
    // true if the program should continue, false if it should stop
    pub fn next(&mut self) -> bool {
        let opcode = self.data[self.pointer];
        let res = match opcode {
            00001 => {
                let idx_a = self.data[self.pointer + 1] as usize;
                let idx_b = self.data[self.pointer + 2] as usize;
                let idx_c = self.data[self.pointer + 3] as usize;
                let val = self.data[idx_a] + self.data[idx_b];
                self.data[idx_c] = val;
                self.pointer += 4;
                true
            }
            00101 => {
                let idx_a = self.data[self.pointer + 1];
                let idx_b = self.data[self.pointer + 2] as usize;
                let idx_c = self.data[self.pointer + 3] as usize;
                let val = idx_a + self.data[idx_b];
                self.data[idx_c] = val;
                self.pointer += 4;
                true
            }
            01001 => {
                let idx_a = self.data[self.pointer + 1] as usize;
                let idx_b = self.data[self.pointer + 2];
                let idx_c = self.data[self.pointer + 3] as usize;
                let val = self.data[idx_a] + idx_b;
                self.data[idx_c] = val;
                self.pointer += 4;
                true
            }
            01101 => {
                let idx_a = self.data[self.pointer + 1];
                let idx_b = self.data[self.pointer + 2];
                let idx_c = self.data[self.pointer + 3] as usize;
                let val = idx_a + idx_b;
                self.data[idx_c] = val;
                self.pointer += 4;
                true
            }
            00002 => {
                let idx_a = self.data[self.pointer + 1] as usize;
                let idx_b = self.data[self.pointer + 2] as usize;
                let idx_c = self.data[self.pointer + 3] as usize;
                let val = self.data[idx_a] * self.data[idx_b];
                self.data[idx_c] = val;
                self.pointer += 4;
                true
            }
            00102 => {
                let idx_a = self.data[self.pointer + 1];
                let idx_b = self.data[self.pointer + 2] as usize;
                let idx_c = self.data[self.pointer + 3] as usize;
                let val = idx_a * self.data[idx_b];
                self.data[idx_c] = val;
                self.pointer += 4;
                true
            }
            01002 => {
                let idx_a = self.data[self.pointer + 1] as usize;
                let idx_b = self.data[self.pointer + 2];
                let idx_c = self.data[self.pointer + 3] as usize;
                let val = self.data[idx_a] * idx_b;
                self.data[idx_c] = val;
                self.pointer += 4;
                true
            }
            01102 => {
                let idx_a = self.data[self.pointer + 1];
                let idx_b = self.data[self.pointer + 2];
                let idx_c = self.data[self.pointer + 3] as usize;
                let val = idx_a * idx_b;
                self.data[idx_c] = val;
                self.pointer += 4;
                true
            }
            3 => {
                let idx_a = self.data[self.pointer + 1] as usize;
                self.data[idx_a] = self.input;
                self.pointer += 2;
                true
            }
            4 => {
                let idx_a = self.data[self.pointer + 1] as usize;
                self.output = self.data[idx_a];
                self.pointer += 2;
                true
            }
            104 => {
                let idx_a = self.data[self.pointer + 1];
                self.output = idx_a;
                self.pointer += 2;
                true
            }
            99 => false,
            _ => panic!("you dun goofed"),
        };
        res
    }
}

#[aoc(day5, part1)]
/// Solves part one by running the intcode program and
/// returning the diagnostic code
fn part_one(input: &[i32]) -> i32 {
    let mut program = Program::new(input.to_vec(), 1);
    program.run_diagnostics()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn opcode_one_and_two() {
        let mut program = Program::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], 1);
        assert!(program.next());
        assert_eq!(program.pointer, 4);
        assert_eq!(
            program.data,
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert!(program.next());
        assert_eq!(program.pointer, 8);
        assert_eq!(
            program.data,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert!(!program.next());
    }

    #[test]
    fn simple_diagnostics() {
        let mut program = Program::new(vec![3, 0, 4, 0, 99], 23);
        assert_eq!(program.run_diagnostics(), 23);
    }

    #[test]
    fn negatives_allowed() {
        let mut program = Program::new(vec![1101, 100, -1, 4, 0], 23);
        assert_eq!(program.run_diagnostics(), 0);
    }
}
