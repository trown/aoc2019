use permutohedron::LexicalPermutation;

/// Parses each line to be an i32
#[aoc_generator(day7)]
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
    /// Output from program
    output: Vec<i32>,
    /// Code pointer
    pointer: usize,
    waiting: bool,
}

#[derive(Debug)]
pub struct Amplifier {
    /// Label for debugging which amplifier this is
    label: String,
    /// Program the amplifier runs
    program: Program,
    /// Whether the program has more instructions
    more: bool,
}

impl Amplifier {
    pub fn new(label: String, output: Vec<i32>, program: &Program) -> Self {
        Amplifier {
            label,
            program: Program::new(program.data.clone(), output),
            more: true,
        }
    }
}

impl Program {
    pub fn new(data: Vec<i32>, output: Vec<i32>) -> Self {
        Program {
            data,
            output,
            pointer: 0,
            waiting: false,
        }
    }

    pub fn read(&mut self, mode: &u8, ptr: usize) -> i32 {
        if mode == &0u8 {
            self.data[self.data[ptr] as usize]
        } else {
            self.data[ptr]
        }
    }

    pub fn calc_thrust(&mut self, phases: &[i32]) -> i32 {
        let mut amp_a = Amplifier::new("a".to_string(), vec![phases[1]], self);
        let mut amp_b = Amplifier::new("b".to_string(), vec![phases[2]], self);
        let mut amp_c = Amplifier::new("c".to_string(), vec![phases[3]], self);
        let mut amp_d = Amplifier::new("d".to_string(), vec![phases[4]], self);
        let mut amp_e = Amplifier::new("e".to_string(), vec![0, phases[0]], self);
        while amp_a.more || amp_b.more || amp_c.more || amp_d.more || amp_e.more {
            amp_a.program.waiting = false;
            while amp_a.more && !amp_a.program.waiting {
                amp_a.more = amp_a.program.next(&mut amp_e.program.output);
            }
            amp_b.program.waiting = false;
            while amp_b.more && !amp_b.program.waiting {
                amp_b.more = amp_b.program.next(&mut amp_a.program.output);
            }
            amp_c.program.waiting = false;
            while amp_c.more && !amp_c.program.waiting {
                amp_c.more = amp_c.program.next(&mut amp_b.program.output);
            }
            amp_d.program.waiting = false;
            while amp_d.more && !amp_d.program.waiting {
                amp_d.more = amp_d.program.next(&mut amp_c.program.output);
            }
            amp_e.program.waiting = false;
            while amp_e.more && !amp_e.program.waiting {
                amp_e.more = amp_e.program.next(&mut amp_d.program.output);
            }
        }
        amp_e.program.output.reverse();
        amp_e.program.output.pop().unwrap()
    }

    pub fn max_thruster(&mut self, phases: &mut Vec<i32>) -> i32 {
        let mut max_thrust = self.calc_thrust(&phases);

        while phases.next_permutation() {
            let thrust = self.calc_thrust(&phases);
            if thrust > max_thrust {
                max_thrust = thrust;
            }
        }
        max_thrust
    }

    // Continues the execution of the program, returning
    // true if the program should continue, false if it should stop
    pub fn next(&mut self, input: &mut Vec<i32>) -> bool {
        let instruction = format!("{:0>6}", &self.data[self.pointer]);
        let opcode = &instruction[4..6];
        let mode_a = &instruction[3..4].parse::<u8>().unwrap();
        let mode_b = &instruction[2..3].parse::<u8>().unwrap();
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
                match input.last() {
                    Some(_) => {
                        self.data[idx_a] = input.pop().unwrap();
                        self.pointer += 2;
                    }
                    None => {
                        self.waiting = true;
                    }
                }
                true
            }
            "04" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                self.output.reverse();
                self.output.push(read_a);
                self.output.reverse();
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

#[aoc(day7, part1)]
fn part_one(input: &[i32]) -> i32 {
    let mut program = Program::new(input.to_vec(), Vec::new());
    let mut phases = vec![0, 1, 2, 3, 4];
    program.max_thruster(&mut phases)
}

#[aoc(day7, part2)]
fn part_two(input: &[i32]) -> i32 {
    let mut program = Program::new(input.to_vec(), Vec::new());
    let mut phases = vec![5, 6, 7, 8, 9];
    program.max_thruster(&mut phases)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day_7_part_1() {
        let mut program = Program::new(
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            Vec::new(),
        );
        let mut phases = vec![0, 1, 2, 3, 4];
        assert_eq!(program.max_thruster(&mut phases), 43210);
        program = Program::new(
            vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            Vec::new(),
        );
        phases = vec![0, 1, 2, 3, 4];
        assert_eq!(program.max_thruster(&mut phases), 54321);
        program = Program::new(
            vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ],
            Vec::new(),
        );
        phases = vec![0, 1, 2, 3, 4];
        assert_eq!(program.max_thruster(&mut phases), 65210);
    }

    #[test]
    fn day_7_part_2() {
        let mut program = Program::new(
            vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5,
            ],
            Vec::new(),
        );
        let mut phases = vec![5, 6, 7, 8, 9];
        assert_eq!(program.max_thruster(&mut phases), 139629729);
        program = Program::new(
            vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
            ],
            Vec::new(),
        );
        phases = vec![5, 6, 7, 8, 9];
        assert_eq!(program.max_thruster(&mut phases), 18216);
    }
}
