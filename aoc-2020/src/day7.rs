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

type Graph = HashMap<String, HashSet<String>>;

/*
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
 */
#[derive(Debug, Clone)]
struct Bag {
    colour: String,
    // (colour, count)
    contains: Vec<(String, u32)>,
}

fn parse_contains(capture: regex::Captures) -> Option<(String, u32)> {
    let count: u32 = capture.get(1)?.as_str().parse().ok()?;
    let bag = capture.get(2)?.as_str().to_string();
    Some((bag, count))
}

fn parse_line(line: String, re: &regex::Regex, re2: &regex::Regex) -> Option<Bag> {
    let captures = re.captures(&line)?;
    let colour = captures.get(1)?.as_str().to_string();
    if let Some(_) = captures.get(6) {
        Some(Bag {
            colour,
            contains: vec![],
        })
    } else {
        let contains = captures
            .get(3)?
            .as_str()
            .split(", ")
            .flat_map(|x| parse_contains(re2.captures(x).unwrap()))
            .collect();
        Some(Bag { colour, contains })
    }
}

fn build_graph(bags: Vec<Bag>) -> Graph {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::with_capacity(bags.len());
    for bag in bags {
        for contain in bag.contains {
            match graph.get_mut(&contain.0) {
                Some(producers) => {
                    producers.insert(bag.colour.clone());
                }
                None => {
                    let mut v = HashSet::new();
                    v.insert(bag.colour.clone());
                    graph.insert(contain.0, v);
                }
            }
        }
    }
    graph
}

fn part1(graph: &Graph) -> usize {
    let mut handled: HashSet<String> = HashSet::new();
    let mut queue: Vec<String> = Vec::new();
    queue.push("shiny gold".to_string());
    while let Some(value) = queue.pop() {
        if let Some(parents) = graph.get(&value) {
            parents
                .iter()
                .filter(|v| !handled.contains(v.to_owned()))
                .for_each(|v| queue.push(v.to_owned()));
        }
        handled.insert(value);
    }
    handled.len() - 1
}

fn part_2(key: &String, graph: &HashMap<String, HashSet<(String, u32)>>) -> u32 {
    graph
        .get(key)
        .iter()
        .flat_map(|x| x.iter())
        .map(|(colour, count)| count * (1 + part_2(colour, graph)))
        .sum()
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let re = regex::Regex::new(
        r"^([a-zA-Z ]+) bags contain ((( ?(\d+) ?[a-zA-Z ]+ bags?,?)*)|(no other bags)).$",
    )
    .unwrap();
    let re2 = regex::Regex::new(r"^(\d+) ([a-zA-Z ]+) bags?$").unwrap();
    let input: Vec<Bag> = reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| parse_line(x, &re, &re2))
        .flatten()
        .collect();
    let graph = build_graph(input.clone());
    println!("Part 1 = {:?}", part1(&graph));
    let tree: HashMap<String, HashSet<(String, u32)>> = input
        .into_iter()
        .map(|bag| (bag.colour, bag.contains.into_iter().collect()))
        .collect();
    println!("Part 2 = {:?}", part_2(&"shiny gold".to_string(), &tree));
}
