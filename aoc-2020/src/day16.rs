use itertools::Itertools;
use structopt::StructOpt;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

type Ticket = Vec<u32>;

#[derive(Debug)]
struct Input {
    ranges: Vec<((u32, u32), (u32, u32))>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
    departure_fields: HashSet<usize>,
}

fn parse_input(mut reader: BufReader<File>) -> Input {
    let mut ranges: Vec<((u32, u32), (u32, u32))> = Vec::new();
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let re_pairs = regex::Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut field_number = 0;
    let mut departure_fields = HashSet::new();
    while line.as_str().trim() != "" {
        let captures = re_pairs
            .captures(&line.trim())
            .expect("failed to parse pairs.");
        let departure_field = captures.get(1).unwrap().as_str().starts_with("departure");
        if departure_field {
            departure_fields.insert(field_number);
        }
        let p1_low: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let p1_high: u32 = captures.get(3).unwrap().as_str().parse().unwrap();
        let p2_low: u32 = captures.get(4).unwrap().as_str().parse().unwrap();
        let p2_high: u32 = captures.get(5).unwrap().as_str().parse().unwrap();
        ranges.push(((p1_low, p1_high), (p2_low, p2_high)));
        line.clear();
        reader.read_line(&mut line).unwrap();
        field_number += 1;
    }
    reader.read_line(&mut line).unwrap();
    assert_eq!(line.as_str().trim(), "your ticket:");
    line.clear();
    reader.read_line(&mut line).unwrap();
    let ticket: Ticket = line.trim().split(",").map(|d| d.parse().unwrap()).collect();
    reader.read_line(&mut line).unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();
    assert_eq!(line.as_str().trim(), "nearby tickets:");
    line.clear();
    let mut nearby_tickets = Vec::new();
    while let Ok(_) = reader.read_line(&mut line) {
        if line.trim() == "" {
            break;
        }
        let t: Ticket = line.trim().split(",").map(|d| d.parse().unwrap()).collect();
        nearby_tickets.push(t);
        line.clear();
    }
    Input {
        ranges,
        ticket,
        nearby_tickets,
        departure_fields,
    }
}

fn part_1(input: &Input) -> u32 {
    // pre-process
    let mut quick_check_map = HashSet::new();
    for ((p1l, p1h), (p2l, p2h)) in &input.ranges {
        for n in *p1l..=*p1h {
            quick_check_map.insert(n);
        }

        for n in *p2l..=*p2h {
            quick_check_map.insert(n);
        }
    }

    let sum_invalid_fields = input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|x| !quick_check_map.contains(x))
        .sum();
    sum_invalid_fields
}

fn get_permutations(
    i: usize,
    order: &Vec<usize>,
    global_options: &HashMap<usize, Vec<usize>>,
    used: HashSet<usize>,
    calculated: HashMap<usize, usize>,
) -> HashSet<Vec<(usize, usize)>> {
    if i == order.len() {
        let item = calculated.into_iter().collect();
        let mut hs = HashSet::new();
        hs.insert(item);
        return hs;
    }
    let n = order[i];

    let mut res: HashSet<Vec<(usize, usize)>> = HashSet::new();
    let options: HashSet<usize> = global_options
        .get(&n)
        .unwrap()
        .iter()
        .filter(|v| !used.contains(v))
        .cloned()
        .collect();
    for option in options {
        let mut t_used = used.clone();
        t_used.insert(option);
        let mut local_p = calculated.clone();
        local_p.insert(n, option);
        for p in get_permutations(i + 1, &order, global_options, t_used, local_p) {
            res.insert(p);
        }
    }
    res
}

fn part_2(input: &Input) -> u64 {
    // get filtered tickets.
    let tickets = {
        let mut quick_check_map = HashSet::new();
        for ((p1l, p1h), (p2l, p2h)) in &input.ranges {
            for n in *p1l..=*p1h {
                quick_check_map.insert(n);
            }

            for n in *p2l..=*p2h {
                quick_check_map.insert(n);
            }
        }
        let tickets: Vec<&Ticket> = input
            .nearby_tickets
            .iter()
            .filter(|ticket| {
                let count = ticket
                    .iter()
                    .filter(|v| !quick_check_map.contains(v))
                    .count();
                count == 0
            })
            .collect();
        tickets
    };
    // lets transpose the tickets so that we have a series of vectors for each ticket field.
    let transpose: Vec<Vec<u32>> = (0..tickets[0].len())
        .map(|i| tickets.iter().map(|t| t[i]).collect())
        .collect();

    let permissible_columns: Vec<(usize, Vec<usize>)> = {
        let mut res: HashMap<usize, Vec<usize>> = HashMap::new();
        let range_vec_valid = |range: &((u32, u32), (u32, u32)), values: &Vec<u32>| {
            let value_valid = |v: u32| {
                (range.0 .0 <= v && v <= range.0 .1) || (range.1 .0 <= v && v <= range.1 .1)
            };
            values.iter().filter(|v| !value_valid(**v)).count() == 0
        };
        for (range_i, range) in input.ranges.iter().enumerate() {
            for (i, column) in transpose.iter().enumerate() {
                if range_vec_valid(range, column) {
                    res.entry(range_i)
                        .and_modify(|e| e.push(i))
                        .or_insert(vec![i]);
                }
            }
        }
        res.into_iter()
            .sorted_by(|a, b| a.1.len().cmp(&b.1.len()))
            .collect()
    };

    let order = permissible_columns.iter().map(|(i, _)| *i).collect();
    let hm: HashMap<usize, Vec<usize>> = permissible_columns.iter().cloned().collect();

    let combinations = get_permutations(0, &order, &hm, HashSet::new(), HashMap::new());
    for combination in combinations {
        let product = combination
            .iter()
            .filter(|(f, _)| input.departure_fields.contains(f))
            .map(|(_, t)| t)
            .map(|c| input.ticket[*c] as u64)
            .product();
        return product;
    }
    panic!("argh");
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let input = parse_input(reader);
    println!("Part 1 = {}", part_1(&input));
    println!("Part 2 = {}", part_2(&input));
}
