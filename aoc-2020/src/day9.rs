use structopt::StructOpt;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn first_constraint(values: &[i64]) -> bool {
    let (target, search_space) = values.split_last().unwrap();
    let available: HashSet<i64> = search_space.iter().cloned().collect();
    for v1 in &available {
        let v2 = target - v1;
        if *v1 != v2 && available.contains(&v2) {
            return true;
        }
    }

    false
}

fn part_1(numbers: &Vec<i64>) -> Option<i64> {
    numbers
        .windows(26)
        .filter(|window| !first_constraint(window))
        .flat_map(|w| w.last().into_iter())
        .map(|x| *x)
        .next()
}

fn part_2(numbers: &Vec<i64>, target: i64) -> i64 {
    let mut lower = 0;
    let mut upper = 1;
    let mut sum = numbers.get(lower).unwrap() + numbers.get(upper).unwrap();

    while upper < numbers.len() - 1 {
        if sum < target {
            upper += 1;
            sum += numbers.get(upper).unwrap();
        } else if sum > target {
            lower += 1;
            upper = lower + 1;
            sum = numbers.get(lower).unwrap() + numbers.get(upper).unwrap();
        }
        if sum == target {
            let min = numbers[lower..=upper].iter().min().unwrap();
            let max = numbers[lower..=upper].iter().max().unwrap();
            return min + max;
        }
    }
    return -1;
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let input: Vec<i64> = reader
        .lines()
        .map(|x| x.unwrap())
        .map(|l| l.parse().unwrap())
        .collect();
    let target = part_1(&input).unwrap();
    println!("{:?}", target);
    println!("{:?}", part_2(&input, target));
}
