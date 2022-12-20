use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    List(Vec<Item>),
    Number(u32),
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Number(i) => format!("{}", i),
            Self::List(items) => format!(
                "[{}]",
                items
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        };
        write!(f, "{}", s)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = match (self, other) {
            (Item::Number(l), Item::Number(r)) => l.partial_cmp(r),
            (Item::List(left), Item::List(right)) => {
                for (l, r) in left.iter().zip(right) {
                    if l < r {
                        return Some(Ordering::Less);
                    }
                    if r < l {
                        return Some(Ordering::Greater);
                    }
                }
                return left.len().partial_cmp(&right.len());
            }
            (Item::Number(n), Item::List(_)) => {
                Item::List(vec![Item::Number(*n)]).partial_cmp(other)
            }
            (Item::List(_), Item::Number(n)) => {
                self.partial_cmp(&Item::List(vec![Item::Number(*n)]))
            }
        };
        res
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_str(mut chars: &mut Peekable<Chars>) -> Option<Item> {
    let next = *chars.peek().unwrap();
    if next == '[' {
        chars.next();
        // parse array
        let mut items = Vec::new();
        while *chars.peek().unwrap() != ']' {
            if let Some(item) = parse_str(&mut chars) {
                items.push(item);
            }
        }
        chars.next();
        Some(Item::List(items))
    } else if next == ',' {
        // consume it
        chars.next();
        None
    } else {
        // we have a digit
        let mut num = String::new();
        while !vec![']', ','].contains(chars.peek().unwrap()) {
            num.push(chars.next().unwrap());
        }
        Some(Item::Number(u32::from_str_radix(&num, 10).unwrap()))
    }
}

impl From<&str> for Item {
    fn from(s: &str) -> Self {
        parse_str(&mut s.chars().peekable()).unwrap()
    }
}

fn read_pair(pair: &str) -> (Item, Item) {
    let mut it = pair.split("\n");
    let f = it.next().unwrap().into();
    let s = it.next().unwrap().into();
    (f, s)
}

fn read_input(input: &str) -> Vec<(Item, Item)> {
    input.split("\n\n").map(read_pair).collect()
}

fn solve_1(input: &[(Item, Item)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (f, s))| f < s)
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve_2(mut input: Vec<(Item, Item)>) -> usize {
    let i1 = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let i2 = Item::List(vec![Item::List(vec![Item::Number(6)])]);
    input.push((i1.clone(), i2.clone()));
    input.sort();
    println!("{:?}", input);
    input
        .iter()
        .map(|(f, s)| vec![f, s])
        .flatten()
        .enumerate()
        .filter(|(_, i)| *i == &i1 || *i == &i2)
        .map(|i| i.0 + 1)
        .product()
}

fn solve(lines: &str) -> (usize, usize) {
    let input = read_input(lines);
    (solve_1(&input), solve_2(input))
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
        assert_eq!(solve(input), (13, 140));
    }
}
