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

struct Schedule {
    earliest_dep: u32,
    bus_ids: Vec<u32>,
}

fn parse_input(mut reader: BufReader<File>) -> Schedule {
    let mut raw_dep_time = String::new();
    reader.read_line(&mut raw_dep_time).unwrap();
    let earliest_dep: u32 = raw_dep_time.trim().parse().unwrap();
    let mut csv_ids = String::new();
    reader.read_line(&mut csv_ids).unwrap();
    let bus_ids = csv_ids
        .replace("x", "0")
        .split(",")
        .map(|id| id.trim().parse().unwrap())
        .collect();

    Schedule {
        earliest_dep,
        bus_ids,
    }
}

fn part_1(schedule: &Schedule) -> u32 {
    let mut id: u32 = 0;
    let mut min_diff = u32::MAX;
    for n in &schedule.bus_ids {
        if *n == 0 {
            continue;
        }
        let diff = (1 + (schedule.earliest_dep / n)) * n - schedule.earliest_dep;
        if diff < min_diff {
            min_diff = diff;
            id = *n;
        }
    }
    id * min_diff
}

// Start sourced from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
// End sourced from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust

fn part_2(schedule: Schedule) -> i64 {
    let mut residues = Vec::new();
    let mut modulii = Vec::new();
    for (i, id) in schedule.bus_ids.iter().enumerate() {
        if *id == 0 {
            continue;
        }
        residues.push(*id as i64 - i as i64);
        modulii.push(*id as i64);
    }
    chinese_remainder(&residues, &modulii).unwrap()
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let input = parse_input(reader);
    println!("Part 1 = {}", part_1(&input));
    println!("Part 2 = {}", part_2(input));
}
