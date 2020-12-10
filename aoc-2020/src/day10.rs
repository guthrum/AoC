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

fn part_1(numbers: &Vec<u64>) -> u64 {
    let mut jolt_diffs: [u64; 3] = [0; 3];
    for pairs in numbers.windows(2) {
        let l = pairs[0];
        let u = pairs[1];
        let idx = ((u as i64) - (l as i64) - 1) as usize;
        jolt_diffs[idx] += 1;
    }
    jolt_diffs[0] * jolt_diffs[2]
}

fn part_2(numbers: &[u64]) -> u64 {
    let mut cumulative_combinations: HashMap<u64, u64> = HashMap::with_capacity(numbers.len());
    cumulative_combinations.insert(0, 1);
    for n in numbers {
        let mut count = 0;
        for i in 1..=3 {
            if *n >= i {
                if let Some(increment) = cumulative_combinations.get(&(n - i)) {
                    count += increment;
                }
            }
        }
        cumulative_combinations.insert(*n, count);
    }
    *cumulative_combinations
        .get(numbers.get(numbers.len() - 2).unwrap())
        .expect("last number is missing.")
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let mut input: Vec<u64> = reader
        .lines()
        .map(|x| x.unwrap())
        .map(|l| l.parse().unwrap())
        .collect();
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
    println!("Part 1 = {}", part_1(&input));
    println!("Part 2 = {}", part_2(&input[1..]));
}
