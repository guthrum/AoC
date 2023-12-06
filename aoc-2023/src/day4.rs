use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::hash::Hash;
use std::str::FromStr;

struct Card {
    id: i32,
    winning: Vec<i32>,
    picked: Vec<i32>,
}

impl Card {
    fn score(&self) -> i64 {
        let correct = self.count();
        if correct == 0 {
            0
        } else {
            2_i64.pow((correct - 1) as u32)
        }
    }

    fn count(&self) -> usize {
        self.picked
            .iter()
            .filter(|p| self.winning.contains(p))
            .count()
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, numbers) = s.split_once(":").unwrap();

        let id = id.strip_prefix("Card ").unwrap().trim();
        let (winning_str, picked_str) = numbers.split_once("|").unwrap();
        let winning = winning_str
            .split_whitespace()
            .map(|n| i32::from_str_radix(n, 10).expect(&format!("failed to parse {n}")))
            .collect();
        let picked = picked_str
            .split_whitespace()
            .map(|n| i32::from_str_radix(n, 10).expect(&format!("failed to parse {n}")))
            .collect();

        Ok(Self {
            id: i32::from_str_radix(id, 10).expect(&format!("failed to parse {id}")),
            winning,
            picked,
        })
    }
}

fn part1(cards: &[Card]) -> i64 {
    cards.iter().map(|c| c.score()).sum()
}

fn part2(cards: &[Card]) -> usize {
    let mut total: HashMap<i32, usize> = HashMap::new();
    for c in cards {
        total.insert(c.id, 1);
    }
    let mut new_cards: HashMap<i32, usize> = HashMap::new();

    for c in cards {
        let count = *total.get(&c.id).unwrap_or(&1_usize) + new_cards.get(&c.id).unwrap_or(&0);

        let matching = c.count();
        for next in (c.id + 1)..=(c.id + matching as i32) {
            *new_cards.entry(next).or_insert(0) += count;
        }
    }

    total.values().sum::<usize>() + new_cards.values().sum::<usize>()
}

fn solve(input: &str) -> (i64, usize) {
    let cards: Vec<Card> = input
        .lines()
        .map(|line| Card::from_str(line))
        .collect::<Result<_, _>>()
        .unwrap();
    let p1 = part1(&cards);
    let p2 = part2(&cards);
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
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(solve(input), (13, 30));
    }
}
