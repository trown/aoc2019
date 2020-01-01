use std::collections::HashMap;

#[aoc_generator(day24)]
fn generator_input(input: &str) -> Vec<Bug> {
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

const PLANET_SIZE: usize = 25;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Bug {
    Dead = 0,
    Alive = 1,
}

#[derive(Debug)]
struct Planet {
    state: Vec<Bug>,
    swap: Vec<Bug>,
    seen: HashMap<i32, Vec<Bug>>,
}

impl Planet {
    fn new(state: Vec<Bug>) -> Self {
        let mut seen = HashMap::new();
        seen.insert(hash_state(&state), state.clone());
        let swap = state.clone();
        Planet { state, swap, seen }
    }
    fn tick(&mut self) -> Option<Vec<Bug>> {
        for pos in 0..PLANET_SIZE {
            let bug = self.state[pos];
            let live_neighbors = self.live_neighbor_count(pos);

            let next_bug = match (bug, live_neighbors) {
                (Bug::Alive, x) if x < 1 => Bug::Dead,
                (Bug::Alive, 1) => Bug::Alive,
                (Bug::Alive, x) if x > 1 => Bug::Dead,
                (Bug::Dead, 1) | (Bug::Dead, 2) => Bug::Alive,
                // All other Bugs remain in the same state.
                (otherwise, _) => otherwise,
            };

            self.swap[pos] = next_bug;
        }
        self.state = self.swap.clone();
        self.seen
            .insert(hash_state(&self.state), self.state.clone())
    }
    fn live_neighbor_count(&self, position: usize) -> u8 {
        match position {
            //corners
            0 => self.state[1] as u8 + self.state[5] as u8,
            4 => self.state[3] as u8 + self.state[9] as u8,
            20 => self.state[15] as u8 + self.state[21] as u8,
            24 => self.state[23] as u8 + self.state[19] as u8,

            // left edge
            x if [5, 10, 15].contains(&x) => {
                self.state[x + 1] as u8 + self.state[x - 5] as u8 + self.state[x + 5] as u8
            }

            // right edge
            x if [9, 14, 19].contains(&x) => {
                self.state[x - 1] as u8 + self.state[x - 5] as u8 + self.state[x + 5] as u8
            }

            // top edge
            x if [1, 2, 3].contains(&x) => {
                self.state[x + 1] as u8 + self.state[x - 1] as u8 + self.state[x + 5] as u8
            }

            // bottom edge
            x if [21, 22, 23].contains(&x) => {
                self.state[x + 1] as u8 + self.state[x - 5] as u8 + self.state[x - 1] as u8
            }

            // middle
            x => {
                self.state[x + 1] as u8
                    + self.state[x - 5] as u8
                    + self.state[x + 5] as u8
                    + self.state[x - 1] as u8
            }
        }
    }
}

fn hash_state(state: &Vec<Bug>) -> i32 {
    let two: i32 = 2;
    state
        .iter()
        .enumerate()
        .fold(0, |acc, (i, bug)| acc + two.pow(i as u32) * *bug as i32)
}

#[aoc(day24, part1)]
fn part_one(input: &[Bug]) -> i32 {
    let mut eris = Planet::new(input.to_vec());
    loop {
        if let Some(state) = eris.tick() {
            return hash_state(&state);
        }
    }
}

#[aoc(day24, part2)]
fn part_two(input: &[Bug]) -> i32 {
    let mut layer0 = input.to_vec();
    layer0[12] = Bug::Dead;
    let mut eris = vec![
        vec![Bug::Dead; 25],
        vec![Bug::Dead; 25],
        layer0,
        vec![Bug::Dead; 25],
        vec![Bug::Dead; 25],
    ];
    for _ in 0..200 {
        let mut new_eris = vec![vec![Bug::Dead; 25]; 3];
        for i in 1..eris.len() - 1 {
            new_eris.push(tick_layers(&eris[i - 1], &eris[i], &eris[i + 1]));
        }
        new_eris.push(vec![Bug::Dead; 25]);
        new_eris.push(vec![Bug::Dead; 25]);
        eris = new_eris;
    }
    eris.into_iter()
        .flat_map(|p| p.into_iter().map(|b| b as i32))
        .sum()
}

fn inner_neighbors(layer: &Vec<Bug>, position: usize) -> i32 {
    match position {
        //corners
        0 => layer[1] as i32 + layer[5] as i32,
        4 => layer[3] as i32 + layer[9] as i32,
        20 => layer[15] as i32 + layer[21] as i32,
        24 => layer[23] as i32 + layer[19] as i32,

        // left edge
        x if [5, 10, 15].contains(&x) => {
            layer[x + 1] as i32 + layer[x - 5] as i32 + layer[x + 5] as i32
        }

        // right edge
        x if [9, 14, 19].contains(&x) => {
            layer[x - 1] as i32 + layer[x - 5] as i32 + layer[x + 5] as i32
        }

        // top edge
        x if [1, 2, 3].contains(&x) => {
            layer[x + 1] as i32 + layer[x - 1] as i32 + layer[x + 5] as i32
        }

        // bottom edge
        x if [21, 22, 23].contains(&x) => {
            layer[x + 1] as i32 + layer[x - 5] as i32 + layer[x - 1] as i32
        }

        x => layer[x + 1] as i32 + layer[x - 5] as i32 + layer[x + 5] as i32 + layer[x - 1] as i32,
    }
}

fn tick_layers(inner: &Vec<Bug>, middle: &Vec<Bug>, outer: &Vec<Bug>) -> Vec<Bug> {
    let mut result = vec![Bug::Dead; 25];
    for i in 0..25 {
        if i == 12 {
            continue;
        }
        let mut neighbors = inner_neighbors(middle, i);
        match i {
            //corners
            0 => {
                neighbors += outer[7] as i32;
                neighbors += outer[11] as i32
            }
            4 => {
                neighbors += outer[7] as i32;
                neighbors += outer[13] as i32
            }
            20 => {
                neighbors += outer[11] as i32;
                neighbors += outer[17] as i32
            }
            24 => {
                neighbors += outer[13] as i32;
                neighbors += outer[17] as i32
            }

            // left edge
            x if [5, 10, 15].contains(&x) => neighbors += outer[11] as i32,

            // right edge
            x if [9, 14, 19].contains(&x) => neighbors += outer[13] as i32,

            // top edge
            x if [1, 2, 3].contains(&x) => neighbors += outer[7] as i32,

            // bottom edge
            x if [21, 22, 23].contains(&x) => neighbors += outer[17] as i32,
            // middle
            7 => {
                for j in &[0, 1, 2, 3, 4] {
                    neighbors += inner[*j as usize] as i32;
                }
            }
            11 => {
                for j in &[0, 5, 10, 15, 20] {
                    neighbors += inner[*j as usize] as i32;
                }
            }
            13 => {
                for j in &[4, 9, 14, 19, 24] {
                    neighbors += inner[*j as usize] as i32;
                }
            }
            17 => {
                for j in &[20, 21, 22, 23, 24] {
                    neighbors += inner[*j as usize] as i32;
                }
            }
            _ => (),
        }

        if middle[i] == Bug::Alive {
            if neighbors == 1 {
                result[i] = Bug::Alive;
            }
        } else {
            if neighbors == 1 || neighbors == 2 {
                result[i] = Bug::Alive;
            }
        }
    }
    result
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_hash_state() {
        let mut bugs = vec![Bug::Dead; 25];
        assert_eq!(hash_state(&bugs), 0);
        bugs[2] = Bug::Alive;
        assert_eq!(hash_state(&bugs), 4);
        bugs[3] = Bug::Alive;
        assert_eq!(hash_state(&bugs), 12);
        bugs[24] = Bug::Alive;
        assert_eq!(hash_state(&bugs), 16777228);
    }
}
