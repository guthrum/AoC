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

fn part_1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|x| x.replace(" ", ""))
        .map(|x| x.chars().collect())
        .map(|x: HashSet<char>| x.len())
        .sum()
}

fn count_common_yes_questions(row: &String) -> usize {
    let mut char_counts: [u32; 26] = [0; 26];
    let mut group_size: u32 = 0;
    for split in row.trim().split(" ") {
        group_size += 1;
        let unique: HashSet<char> = split.chars().collect();
        for c in unique {
            let idx = (c as usize) - 97;
            char_counts[idx] += 1;
        }
    }
    char_counts
        .iter()
        .enumerate()
        .filter(|(_, count)| **count == group_size)
        .count()
}

fn part_2(input: &Vec<String>) -> usize {
    input.iter().map(count_common_yes_questions).sum()
}

fn main() {
    let options = Options::from_args();
    let mut reader = BufReader::new(File::open(options.input).unwrap());
    let mut raw = String::new();
    reader.read_to_string(&mut raw);
    let input: Vec<String> = raw.split("\n\n").map(|x| x.replace("\n", " ")).collect();
    println!("Part 1 = {:?}", part_1(&input));
    println!("Part 2 = {:?}", part_2(&input));
}
