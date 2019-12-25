#[aoc_generator(day24)]
fn generator_input(input: &str) -> Vec<Bug> {
    //let mut bugs = Vec::new();
    input
        .chars()
        .filter(|a| a.to_string().as_bytes() != &[b'\n'])
        .map(|a| match a.to_string().as_bytes() {
            &[b'.'] => Bug::Dead,
            &[b'#'] => Bug::Alive,
            _ => panic!("Not a bug!"),
        })
        .collect()
}

#[derive(Debug)]
enum Bug {
    Dead = 0,
    Alive = 1,
}

#[aoc(day24, part1)]
fn part_one(input: &[Bug]) -> i32 {
    // Implement tick function
    // Implement find neighbors function
    // Hash each planet state as a sum of powers of 2
    // Loop tick function
    // Find first planet state that is seen twice and return its key
    println!("{:?}", input);
    0
}
