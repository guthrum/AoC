use structopt::StructOpt;

use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn convert_to_num(raw: &str, lower_char: char, upper_char: char) -> u8 {
    let mut upper_bound = (2 << (raw.len() - 1)) - 1;
    let mut lower_bound = 0;
    for c in raw.chars() {
        if c == lower_char {
            upper_bound = lower_bound + ((upper_bound - lower_bound) / 2);
        } else if c == upper_char {
            lower_bound = lower_bound + ((upper_bound - lower_bound) / 2);
        } else {
            panic!("unexpected char {}", c);
        }
    }

    upper_bound
}

#[derive(Debug, Eq)]
struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn id(&self) -> u16 {
        (self.row as u16) * 8 + (self.column as u16)
    }

    fn row(&self) -> u8 {
        self.row
    }
}

impl From<String> for Seat {
    fn from(raw: String) -> Self {
        Seat {
            row: convert_to_num(&raw[..7], 'F', 'B'),
            column: convert_to_num(&raw[7..10], 'L', 'R'),
        }
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

fn part_1(seats: &Vec<Seat>) -> u16 {
    seats.iter().map(|x| x.id()).max().unwrap_or(0)
}

fn part_2(seats: Vec<Seat>) -> u16 {
    let mut filtered_seats: Vec<Seat> = seats
        .into_iter()
        .filter(|s| s.row() != 0 && s.row() != 127)
        .collect();
    filtered_seats.sort();
    for pos in 1..filtered_seats.len() - 1 {
        let seat = filtered_seats.get(pos).unwrap();
        let prev = filtered_seats.get(pos - 1).unwrap();
        if seat.id() - prev.id() == 2 {
            return seat.id() - 1;
        }
    }
    0
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let input: Vec<Seat> = reader.lines().map(|l| l.unwrap().into()).collect();
    println!("Part 1 = {:?}", part_1(&input));
    println!("Part 2 = {:?}", part_2(input));
}
