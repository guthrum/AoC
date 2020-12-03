use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

struct Forest {
    data: Vec<Vec<char>>,
}

impl Forest {
    fn position(&self, mut x: usize, y: usize) -> Option<char> {
        x = x % self.data.get(y)?.len();
        let v = self.data.get(y)?.get(x)?;
        Some(*v)
    }

    fn height(&self) -> usize {
        self.data.len()
    }
}

fn count_trees(forest: &Forest, delta_x: usize, delta_y: usize) -> usize {
    let mut count = 0;
    for step in 1..(forest.height() / delta_y) {
        let x = step * delta_x;
        let y = step * delta_y;
        count += match forest.position(x, y) {
            Some('#') => 1,
            _ => 0,
        };
    }
    count
}

fn part_1(data: &Forest) -> usize {
    count_trees(data, 3, 1)
}

fn part_2(data: &Forest) -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, (dx, dy)| acc * count_trees(data, *dx, *dy))
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let raw: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.chars().collect())
        .collect();
    let input = Forest { data: raw };
    println!("Part 1 = {:?}", part_1(&input));
    println!("Part 2 = {:?}", part_2(&input));
}
