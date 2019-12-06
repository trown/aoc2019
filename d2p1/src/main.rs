use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/* An Intcode program is a list of integers separated by commas (like 1,0,0,3,99).
To run one, start by looking at the first integer (called position 0).
Here, you will find an opcode - either 1, 2, or 99.
The opcode indicates what to do; for example, 99 means that the program is finished and should immediately halt.
Encountering an unknown opcode means something went wrong.

Opcode 1 adds together numbers read from two positions and stores the result in a third position.
The three integers immediately after the opcode tell you these three positions -
the first two indicate the positions from which you should read the input values, and the third indicates
the position at which the output should be stored.

For example, if your Intcode computer encounters 1,10,20,30, it should read the values at positions 10 and 20,
add those values, and then overwrite the value at position 30 with their sum.

Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them. Again, the three
integers after the opcode indicate where the inputs and outputs are, not their values.

Once you're done processing an opcode, move to the next one by stepping forward 4 positions.

EASY: 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
*/
fn run(int_code: &mut Vec<usize>) {
    let mut cursor = 0;
    while int_code[cursor] != 99 {
        println!("cursor {}", cursor);
        println!("int_code {:?}", int_code);
        match int_code[cursor] {
            1 => {
                let pos1 = int_code[cursor + 1];
                let pos2 = int_code[cursor + 2];
                let update = int_code[cursor + 3];
                int_code[update] = int_code[pos1] + int_code[pos2];
            }
            2 => {
                let pos1 = int_code[cursor + 1];
                let pos2 = int_code[cursor + 2];
                let update = int_code[cursor + 3];
                int_code[update] = int_code[pos1] * int_code[pos2];
            }
            _ => panic!("you dun goofed"),
        }
        cursor = cursor + 4;
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut int_code: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    run(&mut int_code);
    println!("{:?}", int_code);
}
