use std::collections::HashMap;
use std::env::{args, Args};
use std::fmt::format;
use std::fs::read_to_string;
use std::path::Path;

fn read_input(path: &str) -> (Vec<u32>, Vec<u32>) {
    let contents = read_to_string(Path::new(path)).expect("failed to read file");
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line in contents.lines() {
        let (first, second) = line
            .split_once("   ")
            .expect(&format!("line {line} should be split with empty space"));
        v1.push(u32::from_str_radix(first, 10).expect(&format!("failed to parse {first}")));
        v2.push(u32::from_str_radix(second, 10).expect(&format!("failed to parse {second}")));
    }
    (v1, v2)
}

fn part_1(n1: &Vec<u32>, n2: &Vec<u32>) -> u32 {
    let mut c1 = n1.clone();
    c1.sort();
    let mut c2 = n2.clone();
    c2.sort();
    c1.iter().zip(c2).map(|(f, s)| f.abs_diff(s)).sum()
}

fn part_2(n1: &Vec<u32>, n2: &Vec<u32>) -> u32 {
    let mut count2 = HashMap::new();
    for n in n2.iter() {
        *count2.entry(*n).or_insert(0u32) += 1;
    }
    n1.iter()
        .map(|key| key * count2.get(key).unwrap_or(&0))
        .sum()
}

fn main() {
    let args: Vec<String> = args().collect();
    let file = args.get(1).expect("please supply file as first arg");
    let (numbers1, numbers2) = read_input(&file);
    println!("Part 1 = {}", part_1(&numbers1, &numbers2));
    println!("Part 2 = {}", part_2(&numbers1, &numbers2));
}
