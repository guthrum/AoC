use std::collections::HashSet;
use std::fs::read_to_string;

struct Sequence(Vec<i32>);

impl Sequence {
    fn next(&self) -> i32 {
        // find the differences and collect to sequence
        let deltas = self.deltas();
        let mut unique_values = HashSet::new();
        deltas.iter().for_each(|v| {
            unique_values.insert(v);
        });
        if unique_values.len() == 1 && unique_values.contains(&0) {
            *self.0.last().unwrap()
        } else {
            let seq = Sequence(deltas);
            let delta = seq.next();
            self.0.last().unwrap() + delta
        }
    }

    fn deltas(&self) -> Vec<i32> {
        self.0
            .iter()
            .zip(self.0.iter().skip(1))
            .map(|(f, s)| s - f)
            .collect()
    }

    fn prev(&self) -> i32 {
        // find the differences and collect to sequence
        let deltas = self.deltas();
        let mut unique_values = HashSet::new();
        deltas.iter().for_each(|v| {
            unique_values.insert(v);
        });
        if unique_values.len() == 1 && unique_values.contains(&0) {
            *self.0.last().unwrap()
        } else {
            let seq = Sequence(deltas.clone());
            let delta = seq.prev();
            self.0.first().unwrap() - delta
        }
    }
}

impl From<&str> for Sequence {
    fn from(value: &str) -> Self {
        let values = value
            .split_whitespace()
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .collect();
        Self(values)
    }
}

fn solve(input: &str) -> (i32, i32) {
    let sequences: Vec<Sequence> = input.lines().map(Sequence::from).collect();

    let p1 = sequences.iter().map(|seq| seq.next()).sum();

    let p2 = sequences.iter().map(|seq| seq.prev()).sum();

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
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(solve(input), (114, 0));
    }
}
