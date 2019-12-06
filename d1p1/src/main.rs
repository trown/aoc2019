use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

// Fuel required to launch a given module is based on its mass.
// Specifically, to find the fuel required for a module,
// take its mass, divide by three, round down, and subtract 2.
fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let total_fuel = reader.lines().fold(0, |total, mass| {
        total + calculate_fuel(mass.unwrap().parse().unwrap_or(0))
    });
    println!("{}", total_fuel);
}
