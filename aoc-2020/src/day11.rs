use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Seat {
    Occupied,
    Empty,
    Floor,
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            '#' => Seat::Occupied,
            'L' => Seat::Empty,
            '.' => Seat::Floor,
            _ => panic!("unknown char"),
        }
    }
}

#[derive(Debug, Clone)]
struct Seating {
    data: Vec<Vec<Seat>>,
}

impl Seating {
    fn position(&self, x: usize, y: usize) -> Option<Seat> {
        let v = self.data.get(y)?.get(x)?;
        Some(*v)
    }

    fn mut_position(&mut self, x: usize, y: usize) -> Option<&mut Seat> {
        self.data.get_mut(y)?.get_mut(x)
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn width(&self) -> usize {
        self.data.get(0).map(|x| x.len()).unwrap_or(0)
    }

    fn adjacent_occupied_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for (dx, dy) in vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ] {
            let x = (x as i32) + dx;
            let y = (y as i32) + dy;
            if x >= 0 && y >= 0 {
                count += match self.position(x as usize, y as usize) {
                    Some(Seat::Occupied) => 1,
                    _ => 0,
                };
            }
        }
        count
    }

    fn visible_occupied_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for (dx, dy) in vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ] {
            let mut x = (x as i32) + dx;
            let mut y = (y as i32) + dy;
            while x >= 0 && y >= 0 {
                match self.position(x as usize, y as usize) {
                    Some(Seat::Occupied) => {
                        count += 1;
                        break;
                    }
                    Some(Seat::Empty) => break,
                    None => break,
                    _ => {}
                };
                x = (x as i32) + dx;
                y = (y as i32) + dy;
            }
        }
        count
    }

    fn count_in_state(&self, state: Seat) -> usize {
        self.data.iter().flatten().filter(|s| *s == &state).count()
    }

    fn simulate_turn_1(&mut self, previous_state: Seating) -> usize {
        let mut changed = 0;
        for x in 0..self.width() {
            for y in 0..self.height() {
                let pos = self.mut_position(x, y).unwrap();
                let adjacent = previous_state.adjacent_occupied_count(x, y);
                if *pos == Seat::Empty && adjacent == 0 {
                    *pos = Seat::Occupied;
                    changed += 1;
                } else if *pos == Seat::Occupied && adjacent > 3 {
                    *pos = Seat::Empty;
                    changed += 1;
                }
            }
        }
        changed
    }

    fn simulate_turn_2(&mut self, previous_state: Seating) -> usize {
        let mut changed = 0;
        for x in 0..self.width() {
            for y in 0..self.height() {
                let pos = self.mut_position(x, y).unwrap();
                if *pos != Seat::Floor {
                    let adjacent = previous_state.visible_occupied_count(x, y);
                    if *pos == Seat::Empty && adjacent == 0 {
                        *pos = Seat::Occupied;
                        changed += 1;
                    } else if *pos == Seat::Occupied && adjacent > 4 {
                        *pos = Seat::Empty;
                        changed += 1;
                    }
                }
            }
        }
        changed
    }
}

fn part_1(input: &Seating) -> usize {
    let mut seating = input.clone();
    let mut changed = 1;
    while changed != 0 {
        let previous = seating.clone();
        changed = seating.simulate_turn_1(previous);
    }
    seating.count_in_state(Seat::Occupied)
}

fn part_2(input: &Seating) -> usize {
    let mut seating = input.clone();
    let mut changed = 1;
    while changed != 0 {
        let previous = seating.clone();
        changed = seating.simulate_turn_2(previous);
    }
    seating.count_in_state(Seat::Occupied)
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let raw: Vec<Vec<Seat>> = reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.chars().into_iter().map(|x| Seat::from(x)).collect())
        .collect();
    let input = Seating { data: raw };
    println!("Part 1 = {}", part_1(&input));
    println!("Part 1 = {}", part_2(&input));
}
