use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/* The wires twist and turn, but the two wires occasionally cross paths.
To fix the circuit, you need to find the intersection point closest to the central port.
Because the wires are on a grid, use the Manhattan distance "d((a1,a2),(b1,b2)) = |a1 - b1 | + |a2 - b2|" for this measurement.
While the wires do technically cross right at the central port where they both start,
this point does not count, nor does a wire count as crossing with itself.


EASY:

R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83 = distance 159.
*/

fn find_intersections() {}

fn calc_distance() {}

fn main() {
    let file = File::open("easy.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut raw_wire1 = String::new();
    let mut raw_wire2 = String::new();
    reader.read_line(&mut raw_wire1).unwrap();
    reader.read_line(&mut raw_wire2).unwrap();
    let wire1: Vec<&str> = raw_wire1.split(',').collect();
    let wire2: Vec<&str> = raw_wire2.split(',').collect();
    println!("{:?}", wire1);
    println!("{:?}", wire2);
}
