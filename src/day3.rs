use std::collections::HashSet;
use std::error::Error;
use std::ops::Add;
use std::str::FromStr;

type Wire = Vec<WireSegment>;

#[derive(Debug)]
enum WireSegment {
    Right(i32),
    Down(i32),
    Left(i32),
    Up(i32),
}

impl WireSegment {
    fn to_point(&self) -> Point {
        match self {
            WireSegment::Right(len) => Point {
                x: len.clone(),
                y: 0,
            },
            WireSegment::Down(len) => Point {
                x: 0,
                y: len.clone() * -1,
            },
            WireSegment::Left(len) => Point {
                x: len.clone() * -1,
                y: 0,
            },
            WireSegment::Up(len) => Point {
                x: 0,
                y: len.clone(),
            },
        }
    }
}

impl FromStr for WireSegment {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ident = &s[0..=0];
        let rest = s[1..].parse::<i32>()?;
        match ident {
            "R" => Ok(WireSegment::Right(rest)),
            "D" => Ok(WireSegment::Down(rest)),
            "L" => Ok(WireSegment::Left(rest)),
            "U" => Ok(WireSegment::Up(rest)),
            _ => panic!("you dun goofed"),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Parses each line to be a Wire
#[aoc_generator(day3)]
fn generator_input(input: &str) -> Result<Vec<Wire>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|ws| WireSegment::from_str(ws))
                .collect::<Result<Wire, Box<dyn Error>>>()
        })
        .collect()
}

fn get_points(segment: &WireSegment, start: &Point) -> Vec<Point> {
    match segment {
        WireSegment::Right(len) => (0..len.clone())
            .map(|i| start.clone() + Point { x: i, y: 0 })
            .collect(),
        WireSegment::Down(len) => (0..len.clone())
            .map(|i| start.clone() + Point { x: 0, y: i * -1 })
            .collect(),
        WireSegment::Left(len) => (0..len.clone())
            .map(|i| start.clone() + Point { x: i * -1, y: 0 })
            .collect(),
        WireSegment::Up(len) => (0..len.clone())
            .map(|i| start.clone() + Point { x: 0, y: i })
            .collect(),
    }
}

#[aoc(day3, part1)]
/// Solves part one by finding closest intersection to the origin
fn part_one(input: &Vec<Wire>) -> i32 {
    let mut w1 = HashSet::new();
    let mut w2 = HashSet::new();
    let origin = Point { x: 0, y: 0 };
    let mut current = origin.clone();
    for segment in input[0].iter() {
        for point in get_points(&segment, &current) {
            w1.insert(point);
        }
        current = current + segment.to_point();
    }
    current = origin;
    for segment in input[1].iter() {
        for point in get_points(&segment, &current) {
            w2.insert(point);
        }
        current = current + segment.to_point();
    }
    let mut dists: Vec<i32> = w1
        .intersection(&w2)
        .map(|p| p.x.abs() + p.y.abs())
        .collect();
    dists.sort_unstable();
    println!("dists = {:?}", dists);
    dists[1]
}
