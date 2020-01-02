use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_input(path: &str) -> Option<Vec<OrbitRelation>> {
    let f = File::open(path).ok()?;
    let reader = BufReader::new(f);
    Some(
        reader
            .lines()
            .filter_map(Result::ok)
            .map(|x| x.parse::<OrbitRelation>())
            .filter_map(Result::ok)
            .collect(),
    )
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct OrbitRelation {
    com: String,
    orbiter: String,
}

impl FromStr for OrbitRelation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(')').collect();
        if parts.len() == 2 {
            Ok(OrbitRelation {
                com: parts[0].to_owned(),
                orbiter: parts[1].to_owned(),
            })
        } else {
            Err("Not two parts")
        }
    }
}

#[derive(Debug)]
struct System {
    orbital_relations: Vec<OrbitRelation>,
    com: String,
}

impl System {
    fn new(orbital_relations: Vec<OrbitRelation>, com: String) -> Self {
        System {
            orbital_relations,
            com,
        }
    }

    fn solve_part1(&self) -> i32 {
        let com_to_orbitors = {
            let mut name_children: HashMap<&String, Vec<&String>> = HashMap::new();
            for relation in &self.orbital_relations {
                name_children
                    .entry(&relation.com)
                    .or_insert_with(Vec::new)
                    .push(&relation.orbiter);
            }
            name_children
        };

        let mut com_orbit_count: HashMap<&String, i32> = HashMap::new();
        let mut com_stack: VecDeque<&String> = VecDeque::new();
        com_orbit_count.insert(&self.com, 0);
        com_stack.push_back(&self.com);

        while !com_stack.is_empty() {
            let current_com = com_stack.pop_front().unwrap();
            let current_com_count = com_orbit_count
                .get(current_com)
                .expect("no entry found")
                .to_owned();
            match com_to_orbitors.get(current_com) {
                Some(orbitors) => {
                    for orbitor in orbitors {
                        com_stack.push_front(orbitor);
                        com_orbit_count.insert(orbitor, current_com_count + 1);
                    }
                }
                None => {}
            }
        }

        com_orbit_count.values().sum()
    }

    fn solve_part2(&self) -> i32 {
        let san: String = String::from("SAN");
        let you: String = String::from("YOU");

        let com_to_orbitors = {
            let mut name_children: HashMap<&String, Vec<&String>> = HashMap::new();
            for relation in &self.orbital_relations {
                name_children
                    .entry(&relation.com)
                    .or_insert_with(Vec::new)
                    .push(&relation.orbiter);
            }
            name_children
        };

        let mut planet_hops_to_dest: HashMap<&String, (i32, i32)> = HashMap::new();
        let mut com_stack: VecDeque<&String> = VecDeque::new();
        com_stack.push_back(&self.com);

        let mut min_total_distance = std::i64::MAX;

        while !com_stack.is_empty() {
            let mut evaluated_all = true;
            let mut min_distance_san = std::i32::MAX;
            let mut min_distance_you = std::i32::MAX;

            let current_com = com_stack.front().unwrap().to_owned();
            match com_to_orbitors.get(current_com) {
                Some(orbitors) => {
                    for orbitor in orbitors {
                        if orbitor == &&san {
                            min_distance_san = 0;
                        } else if orbitor == &&you {
                            min_distance_you = 0;
                        } else {
                            match planet_hops_to_dest.get(orbitor) {
                                Some((distance_san, distance_you)) => {
                                    if distance_san < &min_distance_san {
                                        min_distance_san = distance_san.to_owned() + 1;
                                    }
                                    if distance_you < &min_distance_you {
                                        min_distance_you = distance_you.to_owned() + 1;
                                    }
                                }
                                None => {
                                    evaluated_all = false;
                                    com_stack.push_front(orbitor);
                                }
                            }
                        }
                    }
                }
                None => {}
            }

            if evaluated_all {
                if (min_distance_you as i64) + (min_distance_san as i64) < min_total_distance {
                    min_total_distance = (min_distance_you as i64) + (min_distance_san as i64);
                }
                planet_hops_to_dest.insert(current_com, (min_distance_san, min_distance_you));
                com_stack.pop_front();
            }
        }

        min_total_distance as i32
    }
}

fn main() {
    let orbit_relations =
        read_input("/home/tim/projects/AoC19/resources/day6input").expect("unable to load input");
    let system = System::new(orbit_relations, "COM".to_owned());
    println!("answer p1 = {}", system.solve_part1());
    println!("answer p2 = {}", system.solve_part2());
}
