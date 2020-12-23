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

fn parse_input(mut reader: BufReader<File>) -> Input {
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        if line.trim() == "" {
            break;
        }

        let mut parts = line.trim().split(": ");
        let rule_id: usize = parts.next().unwrap().parse().unwrap();
        let rule = parts.next().unwrap();
        if rule.contains("\"") {
            let c = rule.replace("\"", "").chars().next().unwrap();
            rules.insert(rule_id, Rule::Character(c));
        } else {
            let r: Vec<Vec<usize>> = rule.split("|")
                .map(|r| r.trim())
                .map(|r| r.split(" ").map(|x| x.parse().unwrap()).collect())
                .collect();
            rules.insert(rule_id, Rule::Conjunction(r));
        }
    }

    let messages: Vec<Vec<char>> = reader.lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();
    Input {
        messages,
        rules,
    }    
}

#[derive(Debug)]
enum Rule {
    Character(char),
    Conjunction(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct Input {
    rules: HashMap<usize, Rule>,
    messages: Vec<Vec<char>>,
}


/// Returns a vector of lenths that can be consumed to match part of the message.
fn matches(rules: &HashMap<usize, Rule>, message: &[char], rule_id: usize, mut path: Vec<usize>) -> HashSet<usize> {
    if message.len() == 0 {
        return HashSet::new();
    }
    path.push(rule_id);

    match rules.get(&rule_id) {
        Some(rule) => {
            match rule {
                Rule::Character(c) => {
                    if message[0] == *c {
                        let mut hs = HashSet::new();
                        hs.insert(1);
                        hs
                    } else {
                        HashSet::new()
                    }
                }
                Rule::Conjunction(conjunctive_rules) => {
                    let mut match_sizes: HashSet<usize> = HashSet::new();
                    for conjunctive_rule in conjunctive_rules {
                        // keeping track of the different amount of message that can be consumed by
                        // the previous rules.
                        let mut cumulative_consumes = HashSet::new();
                        cumulative_consumes.insert(0);
                        for id in conjunctive_rule {
                            // a conjunctive rule is a series of rules that are expected to match in
                            // order, if a single rule fails then we end
                            if cumulative_consumes.is_empty() {
                                break;
                            }
                            let mut local_consumes = HashSet::new();
                            // For each of the possible previous consumption amounts of the message
                            // we try matching the remainder of the message with the current rule
                            // if a success then we record the different amounts that we consume
                            // cumulatively.
                            for offset in cumulative_consumes {
                                for c in matches(rules, &message[offset..], *id, path.clone()) {
                                    local_consumes.insert(offset + c);
                                }
                            }
                            cumulative_consumes = local_consumes;
                        }
                        for c in cumulative_consumes {
                            match_sizes.insert(c);
                        }
                    }
                    match_sizes
                }
            }
        },
        None => HashSet::new()
    }
}

fn part_1(input: &Input) -> usize {
    input.messages.iter()
        .filter(|msg| matches(&input.rules, msg, 0, Vec::new()).contains(&msg.len()))
        .count()
}

fn part_2(mut input: Input) -> usize {
    if let Some(e) = input.rules.get_mut(&8) {
        *e = Rule::Conjunction(vec![vec![42], vec![42, 8]]);
    }

    if let Some(e) = input.rules.get_mut(&11) {
        *e = Rule::Conjunction(vec![vec![42, 31], vec![42, 11, 31]]);
    }

    input.messages.iter()
        .filter(|msg| matches(&input.rules, msg, 0, Vec::new()).contains(&msg.len()))
        .count()
}


fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let input = parse_input(reader);
    println!("Part 1 = {}", part_1(&input));
    println!("Part 2 = {}", part_2(input));
}
