use std::fs::read_to_string;
use std::hash::Hash;
use std::ops::Index;
use std::str::FromStr;

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn total_winning_strategies(&self) -> usize {
        let count = (0..=self.time).filter(|t| self.does_win(*t)).count();
        count
    }

    fn does_win(&self, hold_time: u64) -> bool {
        let time_left = self.time - hold_time;
        let speed = hold_time;
        self.distance < (speed * time_left)
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();
    assert_eq!(times.len(), distances.len());
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_input_2(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .replace(" ", "");
    let distance = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .replace(" ", "");
    Race {
        time: u64::from_str_radix(&time, 10).unwrap(),
        distance: u64::from_str_radix(&distance, 10).unwrap(),
    }
}

fn solve(input: &str) -> (usize, usize) {
    let races = parse_input(input);

    let p1 = races
        .iter()
        .map(|race| race.total_winning_strategies())
        .product();

    let p2 = parse_input_2(input).total_winning_strategies();

    (p1, p2)
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
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(solve(input), (288, 71503));
    }
}
