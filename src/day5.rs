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

    pub fn run_diagnostics(&mut self, debug: bool) -> i32 {
        if debug {
            println!(
                "first 30: {:?}",
                &self.data[self.pointer..self.pointer + 30],
            );
        }
        while self.next() {
            if debug {
                println!(
                    "========================================================================="
                );
                println!(
                    "pointer: {} next_data: {:?}",
                    self.pointer,
                    &self.data[self.pointer..]
                );
            }
        }
        self.output
    }
    pub fn read(&mut self, mode: &u8, ptr: usize) -> i32 {
        if mode == &0u8 {
            self.data[self.data[ptr] as usize]
        } else {
            self.data[ptr]
        }
    }

    // Continues the execution of the program, returning
    // true if the program should continue, false if it should stop
    pub fn next(&mut self) -> bool {
        let instruction = format!("{:0>6}", &self.data[self.pointer]);
        let opcode = &instruction[4..6];
        let mode_a = &instruction[3..4].parse::<u8>().unwrap();
        //println!("{}", mode_a);
        let mode_b = &instruction[2..3].parse::<u8>().unwrap();
        //println!("{}", mode_b);
        let res = match opcode {
            "01" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                let read_b = self.read(mode_b, self.pointer + 2);
                let w_ptr = self.data[self.pointer + 3] as usize;
                self.data[w_ptr] = read_a + read_b;
                self.pointer += 4;
                true
            }
            "02" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                let read_b = self.read(mode_b, self.pointer + 2);
                let w_ptr = self.data[self.pointer + 3] as usize;
                self.data[w_ptr] = read_a * read_b;
                self.pointer += 4;
                true
            }
            "03" => {
                let idx_a = self.data[self.pointer + 1] as usize;
                self.data[idx_a] = self.input;
                self.pointer += 2;
                true
            }
            "04" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                self.output = read_a;
                self.pointer += 2;
                true
            }
            "05" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                let read_b = self.read(mode_b, self.pointer + 2);
                if read_a != 0 {
                    self.pointer = read_b as usize
                } else {
                    self.pointer += 3
                }
                true
            }
            "06" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                let read_b = self.read(mode_b, self.pointer + 2);
                if read_a == 0 {
                    self.pointer = read_b as usize
                } else {
                    self.pointer += 3
                }
                true
            }
            "07" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                let read_b = self.read(mode_b, self.pointer + 2);
                let w_ptr = self.data[self.pointer + 3] as usize;
                if read_a < read_b {
                    self.data[w_ptr] = 1;
                } else {
                    self.data[w_ptr] = 0;
                }
                self.pointer += 4;
                true
            }
            "08" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                let read_b = self.read(mode_b, self.pointer + 2);
                let w_ptr = self.data[self.pointer + 3] as usize;
                if read_a == read_b {
                    self.data[w_ptr] = 1;
                } else {
                    self.data[w_ptr] = 0;
                }
                self.pointer += 4;
                true
            }
            "99" => false,
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
    program.run_diagnostics(false)
}

#[aoc(day5, part2)]
/// Solves part one by running the intcode program and
/// returning the diagnostic code
fn part_two(input: &[i32]) -> i32 {
    let mut program = Program::new(input.to_vec(), 5);
    program.run_diagnostics(true)
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
        assert_eq!(program.run_diagnostics(false), 23);
    }

    #[test]
    fn negatives_allowed() {
        let mut program = Program::new(vec![1101, 100, -1, 4, 0], 23);
        assert_eq!(program.run_diagnostics(false), 0);
        program = Program::new(
            vec![
                3, 28, 1, 28, 6, 6, 1100, 1, 238, 28, 104, 0, 1102, 72, 20, 27, 1001, 27, -1440,
                27, 4, 27, 102, 99, 7, 26, 1001, 224, 5, 224, 99,
            ],
            1,
        );
        assert_eq!(program.run_diagnostics(true), 0);
    }

    #[test]
    fn day_5_part_2() {
        let mut program = Program::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            5,
        );
        assert_eq!(program.run_diagnostics(true), 999);
        program = Program::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            8,
        );
        assert_eq!(program.run_diagnostics(true), 1000);
        program = Program::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            23,
        );
        assert_eq!(program.run_diagnostics(true), 1001);
    }
}
