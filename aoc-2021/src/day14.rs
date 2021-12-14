use std::{
    collections::HashMap,
    fs::read_to_string,
};

fn read_input(path: &str) -> (String, HashMap<(char, char), char>) {
    let raw = read_to_string(path).unwrap();
    let mut lines = raw.lines();
    let polymer = lines.next().unwrap().to_string();
    lines.next().unwrap();
    let mut pairs = HashMap::new();
    while let Some(raw_pair) = lines.next() {
        let mut parts = raw_pair.split(" -> ");
        let p1 = parts.next().unwrap().to_string();
        let p2 = parts.next().unwrap().chars().next().unwrap();
        pairs.insert(
            (
                p1.chars().next().unwrap(),
                p1.chars().skip(1).next().unwrap(),
            ),
            p2,
        );
    }

    (polymer, pairs)
}

fn solve(input: (String, HashMap<(char, char), char>), rounds: u32) -> usize {
    let chain = input.0;
    let mut pair_counts = HashMap::new();
    for (l, r) in chain.chars().zip(chain.chars().skip(1)) {
        pair_counts.insert((l, r), 1);
    }

    for _i in 1..=rounds {
        let mut new_counts = HashMap::new();
        for ((l, r), count) in pair_counts.iter() {
            if let Some(mid) = input.1.get(&(*l, *r)) {
                *new_counts.entry((*l, *mid)).or_insert(0) += count;
                *new_counts.entry((*mid, *r)).or_insert(0) += count;
            }
        }
        pair_counts = new_counts;
    }
    let mut counts = HashMap::new();
    for ((l, _), count) in pair_counts.into_iter() {
        *counts.entry(l).or_insert(0) += count;
    }
    *counts.entry(chain.chars().last().unwrap()).or_insert(0) += 1;

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let input = read_input(&file_path);
    println!("Part 1 = {}", solve(input.clone(), 10));
    println!("Part 2 = {}", solve(input, 40));
}
