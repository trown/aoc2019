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
