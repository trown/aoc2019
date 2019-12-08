/// Parses input to be i32's representing start/end of a range
#[aoc_generator(day4)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split("-")
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

// from https://users.rust-lang.org/t/i32-to-vector-of-i32/18504/11
fn to_digits(input: &i32) -> Box<[u8]> {
    let y = format!("{:#}", input);
    let mut v = y.into_bytes().into_boxed_slice();
    v.iter_mut().for_each(|b| *b -= 48);
    v
}

fn check_pw_rules(pw: &i32) -> bool {
    let digits = to_digits(pw);
    // check not decreasing
    for i in 1..6 {
        if digits[i] < digits[i - 1] {
            return false;
        }
    }
    // check for at least one double
    for i in 1..6 {
        if digits[i] == digits[i - 1] {
            return true;
        }
    }
    false
}

#[aoc(day4, part1)]
/// Solves part one by filtering out all the passwords
/// that dont fit the rules and counting what is left
fn part_one(input: &[i32]) -> usize {
    (input[0]..input[1])
        .filter(|pw| check_pw_rules(pw))
        .collect::<Vec<i32>>()
        .len()
}
