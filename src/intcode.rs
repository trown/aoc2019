//use std::io;

#[derive(Clone, Debug)]
pub struct Program {
    /// Data of the program (parsed input)
    pub data: Vec<i64>,
    /// Output from program
    pub output: Vec<i64>,
    /// Code pointer
    pub pointer: usize,
    pub relative_base: usize,
    pub waiting: bool,
}

impl Program {
    pub fn new(data: Vec<i64>, output: Vec<i64>) -> Self {
        let mut extra_memory = vec![0; 1000];
        let mut p = Program {
            data,
            output,
            pointer: 0,
            relative_base: 0,
            waiting: false,
        };
        p.data.append(&mut extra_memory);
        p
    }

    pub fn read(&mut self, mode: &u8, ptr: usize) -> i64 {
        match mode {
            &0u8 => self.data[self.data[ptr] as usize],
            &1u8 => self.data[ptr],
            &2u8 => self.data[(self.data[ptr] + self.relative_base as i64) as usize],
            _ => panic!("unsupported read mode"),
        }
    }

    pub fn write(&mut self, mode: &u8, ptr: usize, content: i64) {
        let pos;
        match mode {
            &0u8 => pos = self.data[ptr] as usize,
            &2u8 => pos = (self.data[ptr] + self.relative_base as i64) as usize,
            _ => panic!("unsupported read mode"),
        }
        self.data[pos] = content
    }

    pub fn run(&mut self, input: &mut Vec<i64>) {
        while self.next(input) {}
    }

    // Continues the execution of the program, returning
    // true if the program should continue, false if it should stop
    pub fn next(&mut self, input: &mut Vec<i64>) -> bool {
        let instruction = format!("{:0>6}", &self.data[self.pointer]);
        let opcode = &instruction[4..6];
        let mode_a = &instruction[3..4].parse::<u8>().unwrap();
        let mode_b = &instruction[2..3].parse::<u8>().unwrap();
        let mode_c = &instruction[1..2].parse::<u8>().unwrap();
        let res = match opcode {
            "01" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                let read_b = self.read(mode_b, self.pointer + 2);
                self.write(mode_c, self.pointer + 3, read_a + read_b);
                self.pointer += 4;
                true
            }
            "02" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                let read_b = self.read(mode_b, self.pointer + 2);
                self.write(mode_c, self.pointer + 3, read_a * read_b);
                self.pointer += 4;
                true
            }
            "03" => {
                /*let mut input = String::new();
                                 io::stdin().read_line(&mut input).unwrap();
                let n: i64 = input.trim().parse().unwrap();
                self.write(mode_a, self.pointer + 1, n);
                self.pointer += 2;
                println!("{}", input); */
                match input.last() {
                    Some(_) => {
                        self.write(mode_a, self.pointer + 1, input.pop().unwrap());
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
                self.output.push(read_a);
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
                if read_a < read_b {
                    self.write(mode_c, self.pointer + 3, 1);
                } else {
                    self.write(mode_c, self.pointer + 3, 0);
                }
                self.pointer += 4;
                true
            }
            "08" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                let read_b = self.read(mode_b, self.pointer + 2);
                if read_a == read_b {
                    self.write(mode_c, self.pointer + 3, 1);
                } else {
                    self.write(mode_c, self.pointer + 3, 0);
                }
                self.pointer += 4;
                true
            }
            "09" => {
                let read_a = self.read(mode_a, self.pointer + 1);
                self.relative_base += read_a as usize;
                self.pointer += 2;
                true
            }
            "99" => false,
            _ => panic!("Invalid Op Code"),
        };
        res
    }
}
