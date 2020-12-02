use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

/*
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
 */
#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug)]
struct PolicyPassword {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn matches_policy_1(password_policy: &PolicyPassword) -> bool {
    let count = password_policy
        .password
        .chars()
        .into_iter()
        .filter(|c| *c == password_policy.letter)
        .count();
    password_policy.min <= count && count <= password_policy.max
}

fn part_1(data: &Vec<PolicyPassword>) -> usize {
    data.into_iter().filter(|p| matches_policy_1(p)).count()
}

fn matches_policy_2(password_policy: &PolicyPassword) -> bool {
    let chars: Vec<char> = password_policy.password.chars().collect();
    let first = *chars.get(password_policy.min - 1).unwrap() == password_policy.letter;
    let second = *chars.get(password_policy.max - 1).unwrap() == password_policy.letter;
    first ^ second
}

fn part_2(data: &Vec<PolicyPassword>) -> usize {
    data.into_iter().filter(|p| matches_policy_2(p)).count()
}

fn parse_line(line: String, re: &regex::Regex) -> Option<PolicyPassword> {
    let captures = re.captures(&line)?;
    Some(PolicyPassword {
        min: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        max: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        letter: captures.get(3).unwrap().as_str().parse::<char>().unwrap(),
        password: captures.get(4).unwrap().as_str().to_string(),
    })
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let re = regex::Regex::new(r"^(\d+)-(\d+) ([A-Za-z]): ([A-Za-z]*)$").unwrap();
    let input: Vec<PolicyPassword> = reader
        .lines()
        .into_iter()
        .map(|x| x.unwrap())
        .map(|x| parse_line(x, &re).unwrap())
        .collect();
    println!("Part 1 = {:?}", part_1(&input));
    println!("Part 2 = {:?}", part_2(&input));
}
