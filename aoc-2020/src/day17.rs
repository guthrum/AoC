use structopt::StructOpt;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

type Coord = (i64, i64, i64, i64);

fn neighbouring_coords(coord: Coord) -> Vec<Coord> {
    let mut neighbours = Vec::new();
    for x in vec![-1, 0, 1] {
        for y in vec![-1, 0, 1] {
            for z in vec![-1, 0, 1] {
                for w in vec![-1, 0, 1] {
                    if !(x == 0 && y == 0 && z == 0 && w == 0) {
                        neighbours.push((coord.0 + x, coord.1 + y, coord.2 + z, coord.3 + w));
                    }
                }
            }
        }
    }
    neighbours
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State {
    Active,
    Inactive,
}

#[derive(Debug)]
struct PocketDimension {
    state: HashMap<Coord, State>,
}

impl PocketDimension {
    fn state(&self, coord: &Coord) -> State {
        *self.state.get(coord).unwrap_or(&State::Inactive)
    }

    fn bounding_coords(&self) -> (Coord, Coord) {
        let mut lx = 0;
        let mut hx = 0;
        let mut ly = 0;
        let mut hy = 0;
        let mut lz = 0;
        let mut hz = 0;
        let mut lw = 0;
        let mut hw = 0;

        for (x, y, z, w) in self.state.keys() {
            lx = lx.min(x - 1);
            hx = hx.max(x + 1);
            ly = ly.min(y - 1);
            hy = hy.max(y + 1);
            lz = lz.min(z - 1);
            hz = hz.max(z + 1);
            lw = lw.min(w - 1);
            hw = hw.max(w + 1);
        }

        ((lx, ly, lz, lw), (hx, hy, hz, hw))
    }

    fn count_active(&self) -> usize {
        self.state.len()
    }

    fn run_round(&mut self) {
        let mut global_state = self.state.clone();

        let ((lx, ly, lz, lw), (hx, hy, hz, hw)) = self.bounding_coords();

        for x in lx..=hx {
            for y in ly..=hy {
                for z in lz..=hz {
                    for w in lw..=hw {
                        let neighbour_states: HashMap<State, u32> =
                            neighbouring_coords((x, y, z, w))
                                .iter()
                                .map(|c| self.state(c))
                                .fold(HashMap::new(), |mut acc, s| {
                                    *acc.entry(s).or_insert(0) += 1;
                                    acc
                                });
                        let count_ia = *neighbour_states.get(&State::Inactive).unwrap_or(&0);
                        let count_a = *neighbour_states.get(&State::Active).unwrap_or(&0);
                        let new_state = match (self.state(&(x, y, z, w)), count_ia, count_a) {
                            (State::Active, _, 2) => State::Active,
                            (State::Active, _, 3) => State::Active,
                            (State::Inactive, _, 3) => State::Active,
                            _ => State::Inactive,
                        };
                        global_state.remove(&(x, y, z, w));
                        if new_state == State::Active {
                            global_state.insert((x, y, z, w), new_state);
                        }
                    }
                }
            }
        }

        self.state = global_state;
    }
}

impl From<String> for PocketDimension {
    fn from(raw: String) -> Self {
        let mut state = HashMap::new();
        for (y, line) in raw.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        state.insert((x as i64, y as i64, 0, 0), State::Active);
                    }
                    _ => {}
                }
            }
        }

        PocketDimension { state }
    }
}

fn solve(mut input: PocketDimension) -> usize {
    for i in 1..=6 {
        println!(
            "Running round {} current active = {}",
            i,
            input.count_active()
        );
        input.run_round();
    }
    input.count_active()
}

fn main() {
    let options = Options::from_args();
    let mut reader = BufReader::new(File::open(options.input).unwrap());
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let pocket_dimension = PocketDimension::from(input);
    println!("Part 1 = {}", solve(pocket_dimension));
}
