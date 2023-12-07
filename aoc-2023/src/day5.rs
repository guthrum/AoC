use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::Hash;
use std::ops::Index;
use std::str::FromStr;

fn unsafe_int(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap()
}

struct Range {
    from: i64,
    length: i64,
    target_from: i64,
}

impl Range {
    fn contains(&self, i: i64) -> bool {
        self.from <= i && i < self.from + self.length
    }

    fn map(&self, i: i64) -> i64 {
        let offset = i - self.from;
        self.target_from + offset
    }
}

impl Into<Range> for [i64; 3] {
    fn into(self) -> Range {
        Range {
            target_from: self[0],
            from: self[1],
            length: self[2],
        }
    }
}

struct Mapping {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Mapping {
    fn map(&self, value: i64) -> i64 {
        let range = self.ranges.iter().find(|r| r.contains(value));
        if let Some(range) = range {
            range.map(value)
        } else {
            value
        }
    }
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (from, to) = lines
            .next()
            .unwrap()
            .strip_suffix(" map:")
            .unwrap()
            .split_once("-to-")
            .unwrap();

        let ranges = lines
            .map(|l| l.split(" "))
            .map(|mut l| {
                [
                    unsafe_int(l.next().unwrap()),
                    unsafe_int(l.next().unwrap()),
                    unsafe_int(l.next().unwrap()),
                ]
                .into()
            })
            .collect();

        Ok(Self {
            from: from.to_string(),
            to: to.to_string(),
            ranges,
        })
    }
}

struct Mapper {
    mappings: HashMap<String, Mapping>,
}

impl Mapper {
    fn new(mappers: Vec<Mapping>) -> Self {
        let mut mappings = HashMap::with_capacity(mappers.len());
        for m in mappers {
            mappings.insert(m.from.clone(), m);
        }

        Self { mappings }
    }

    fn map_seed(&self, value: i64) -> i64 {
        self.map_chain(value, "seed")
    }

    fn map_chain(&self, value: i64, from: &str) -> i64 {
        if let Some(mapping) = self.mappings.get(from) {
            let mapped_value = mapping.map(value);
            self.map_chain(mapped_value, &mapping.to)
        } else {
            value
        }
    }
}

fn parse_input(input: &str) -> (Vec<i64>, Mapper) {
    let mut groups = input.split("\n\n");
    let seeds = groups
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|v| i64::from_str_radix(v, 10).expect(&format!("failed to parse {v}")))
        .collect();
    let mappings = groups
        .map(|g| Mapping::from_str(g))
        .collect::<Result<_, _>>()
        .unwrap();

    (seeds, Mapper::new(mappings))
}

fn part2(mut v: Vec<i64>, mapper: Mapper) -> i64 {
    let mut min = i64::MAX;
    let mut seeds = v.into_iter().peekable();
    while seeds.peek().is_some() {
        let start = seeds.next().unwrap();
        let count = seeds.next().unwrap();
        println!("processing {count}");
        let m = (start..(start + count))
            .map(|v| mapper.map_seed(v))
            .min()
            .unwrap();
        min = min.min(m);
    }

    min
}

fn solve(input: &str) -> (i64, i64) {
    let (seeds, mapper) = parse_input(input);

    let p1 = seeds
        .iter()
        .map(|seed| mapper.map_seed(*seed))
        .min()
        .unwrap();

    (p1, part2(seeds, mapper))
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_input() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;
        assert_eq!(solve(input), (35, 46));
    }
}
