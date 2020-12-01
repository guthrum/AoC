use structopt::StructOpt;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

/*
 * This approach cannot handle multiple values that are the same, probably would solve that with a
 * hashmap instead or value -> count and then you can keep track of the number of values that have
 * been consumed.
 */

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn find_pair(data: &HashSet<i32>, target: i32) -> Option<(i32, i32)> {
    for value in data {
        if data.contains(&(target - value)) {
            return Some((*value, target - value));
        }
    }
    None
}

fn find_triple(data: &HashSet<i32>, target: i32) -> Option<(i32, i32, i32)> {
    for value in data {
        if let Some(pair) = find_pair(data, target - value) {
            return Some((*value, pair.0, pair.1));
        }
    }
    None
}

fn part_1(data: &HashSet<i32>) -> Option<i32> {
    let pair = find_pair(data, 2020)?;
    Some(pair.0 * pair.1)
}

fn part_2(data: &HashSet<i32>) -> Option<i32> {
    let triple = find_triple(data, 2020)?;
    Some(triple.0 * triple.1 * triple.2)
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let input: HashSet<i32> = reader
        .lines()
        .into_iter()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();
    println!("Part 1 = {}", part_1(&input).unwrap());
    println!("Part 2 = {}", part_2(&input).unwrap());
}
