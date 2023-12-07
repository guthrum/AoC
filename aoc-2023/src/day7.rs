use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq)]
struct Card(char);

impl Card {
    fn value(&self) -> i64 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            // 'J' => 11, // toggle for p1/p2
            _ => panic!(),
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        Self(value)
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandKind {
    fn for_cards(cards: &str) -> Self {
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in cards.chars() {
            *map.entry(c).or_insert(0) += 1
        }
        let mut counts: Vec<usize> = map.values().cloned().collect();
        counts.sort();
        counts.reverse();
        if counts == vec![5] {
            Self::FiveOfAKind
        } else if counts == vec![4, 1] {
            Self::FourOfAKind
        } else if counts == vec![3, 2] {
            Self::FullHouse
        } else if counts == vec![3, 1, 1] {
            Self::ThreeOfAKind
        } else if counts == vec![2, 2, 1] {
            Self::TwoPair
        } else if counts == vec![2, 1, 1, 1] {
            Self::OnePair
        } else if counts == vec![1, 1, 1, 1, 1] {
            Self::HighCard
        } else {
            panic!("invalid")
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand(Vec<Card>, HandKind);

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        assert_eq!(s.len(), 5);
        let mut kind = HandKind::for_cards(&s);
        for c in "J23456789TXQKA".chars() {
            let nk = HandKind::for_cards(&s.replace("J", c.to_string().as_str()));
            if kind < nk {
                kind = nk;
            }
        }
        let cards: Vec<Card> = s.chars().map(|c| c.into()).collect();
        Self(cards, kind)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 == other.1 {
            let d = self
                .0
                .iter()
                .zip(other.0.iter())
                .filter(|(f, s)| f != s)
                .map(|(f, s)| f.value() - s.value())
                .next()
                .unwrap();
            if d < 0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            if self.1 < other.1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<(Hand, u64)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (hand.into(), u64::from_str_radix(bid, 10).unwrap())
        })
        .collect()
}

fn solve(input: &str) -> (u64, usize) {
    let mut hands_and_bids = parse_input(input);
    hands_and_bids.sort_by(|o1, o2| o1.0.cmp(&o2.0));
    let winnings = hands_and_bids
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u64 + 1) * bid)
        .sum();

    (winnings, 0)
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("result = {}", p1);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_input() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(solve(input), (6440, 0));
    }
}
