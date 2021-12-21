use std::fs::read_to_string;

fn is_open(c: char) -> bool {
    c == '{' || c == '(' || c == '<' || c == '['
}

fn expected_closing(c: char) -> char {
    match c {
        '{' => '}',
        '[' => ']',
        '<' => '>',
        '(' => ')',
        _ => panic!("cannot handle input"),
    }
}

struct Line {
    raw: String,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum State {
    Incomplete(String),
    Corrupted(char),
    Complete,
}

impl State {
    fn syntax_error_score(&self) -> u32 {
        match *self {
            Self::Corrupted(c) => match c {
                '}' => 1197,
                ']' => 57,
                '>' => 25137,
                ')' => 3,
                _ => panic!("cannot handle input"),
            },
            _ => 0,
        }
    }

    fn autocomplete_score(&self) -> u64 {
        match self {
            Self::Incomplete(required) => required
                .chars()
                .map(|c| match c {
                    '}' => 3,
                    ']' => 2,
                    '>' => 4,
                    ')' => 1,
                    _ => panic!("cannot handle input"),
                })
                .fold(0, |total, char_score| (total * 5) + char_score),
            _ => 0,
        }
    }
}

impl Line {
    fn new(raw: &str) -> Self {
        Line {
            raw: raw.to_string(),
        }
    }

    fn state(&self) -> State {
        let mut stack = Vec::new();
        for c in self.raw.chars() {
            if is_open(c) {
                stack.push(c);
            } else if let Some(opening) = stack.pop() {
                if expected_closing(opening) != c {
                    return State::Corrupted(c);
                }
            } else {
                panic!("un-handled");
            }
        }
        if stack.is_empty() {
            State::Complete
        } else {
            stack.reverse();
            let closing_str = stack.iter().map(|c| expected_closing(*c)).collect();
            State::Incomplete(closing_str)
        }
    }
}

fn read_input(path: &str) -> Vec<Line> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(Line::new)
        .collect()
}

fn solve(input: &Vec<Line>) -> (u32, u64) {
    let p1 = input.iter().map(|v| v.state().syntax_error_score()).sum();
    let mut auto_complete_score: Vec<u64> = input
        .iter()
        .map(|v| v.state().autocomplete_score())
        .filter(|score| *score > 0)
        .collect();
    auto_complete_score.sort_unstable();
    let p2 = auto_complete_score[auto_complete_score.len() / 2];

    (p1, p2)
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let input = read_input(&file_path);
    let (p1, p2) = solve(&input);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
