use std::collections::HashSet;
use std::error::Error;
use std::hash::{Hash, Hasher};
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
                hops: len.clone(),
            },
            WireSegment::Down(len) => Point {
                x: 0,
                y: len.clone() * -1,
                hops: len.clone(),
            },
            WireSegment::Left(len) => Point {
                x: len.clone() * -1,
                y: 0,
                hops: len.clone(),
            },
            WireSegment::Up(len) => Point {
                x: 0,
                y: len.clone(),
                hops: len.clone(),
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

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    hops: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            hops: self.hops + other.hops,
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
            .map(|i| {
                start.clone()
                    + Point {
                        x: i,
                        y: 0,
                        hops: i,
                    }
            })
            .collect(),
        WireSegment::Down(len) => (0..len.clone())
            .map(|i| {
                start.clone()
                    + Point {
                        x: 0,
                        y: i * -1,
                        hops: i,
                    }
            })
            .collect(),
        WireSegment::Left(len) => (0..len.clone())
            .map(|i| {
                start.clone()
                    + Point {
                        x: i * -1,
                        y: 0,
                        hops: i,
                    }
            })
            .collect(),
        WireSegment::Up(len) => (0..len.clone())
            .map(|i| {
                start.clone()
                    + Point {
                        x: 0,
                        y: i,
                        hops: i,
                    }
            })
            .collect(),
    }
}

#[aoc(day3, part1)]
/// Solves part one by finding closest intersection to the origin
fn part_one(input: &Vec<Wire>) -> i32 {
    let mut w1 = HashSet::new();
    let mut w2 = HashSet::new();
    let origin = Point {
        x: 0,
        y: 0,
        hops: 0,
    };
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

#[aoc(day3, part2)]
/// Solves part two by finding closest intersection to the origin
fn part_two(input: &Vec<Wire>) -> i32 {
    let mut w1 = HashSet::new();
    let mut w2 = HashSet::new();
    let origin = Point {
        x: 0,
        y: 0,
        hops: 0,
    };
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
    let mut dists = Vec::new();
    for p1 in w1 {
        for p2 in &w2 {
            if p1 == p2.clone() {
                dists.push(p1.hops + p2.hops)
            }
        }
    }
    dists.sort_unstable();
    println!("dists = {:?}", dists);
    dists[1]
}
