fn read_input(contents: &str) -> Vec<Range> {
    let parse_range = |v: &str| {
        let (start, end) = v.trim().split_once("-").expect("require 2 ids per line");
        Range {
            start: u64::from_str_radix(start.trim(), 10).expect(&format!("{} invalid u32", start)),
            end: u64::from_str_radix(end.trim(), 10).expect(&format!("{} invalid u32", end)),
        }
    };

    contents.split(",").map(parse_range).collect()
}

fn check_invalid(chars: &[char], split_count: usize) -> bool {
    let step_count = chars.len() / split_count;
    let mut indexes: Vec<_> = (0..chars.len()).step_by(step_count).collect();
    let start = indexes.pop().unwrap();
    for offset in 0..step_count {
        let c = chars[start + offset];
        for i in &indexes {
            if chars[i + offset] != c {
                return false;
            }
        }
    }

    true
}

fn number_invalid_p1(n: u64) -> bool {
    let s = n.to_string();
    if s.len() % 2 != 0 {
        false
    } else {
        let chars: Vec<char> = s.chars().collect();
        check_invalid(&chars, 2)
    }
}

fn number_invalid_p2(n: u64) -> bool {
    let chars: Vec<char> = n.to_string().chars().collect();

    let mut split_n = 2;
    while split_n <= chars.len() {
        if chars.len() % split_n == 0 && check_invalid(&chars, split_n) {
            return true;
        }
        split_n += 1;
    }
    false
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn sum_invalid_contents(&self, f: fn(u64) -> bool) -> u64 {
        (self.start..=self.end).filter(|n| f(*n)).sum()
    }
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("missing file as first argument");
    let contents = std::fs::read_to_string(file).expect("failed to read file");
    let input = read_input(&contents);

    let p1: u64 = input
        .iter()
        .map(|r| r.sum_invalid_contents(number_invalid_p1))
        .sum();
    println!("Part 1 = {}", p1);
    let p2: u64 = input
        .iter()
        .map(|r| r.sum_invalid_contents(number_invalid_p2))
        .sum();
    println!("Part 2 = {}", p2);
}
