use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

impl Digit {
    fn as_u64(&self) -> u64 {
        match *self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
        }
    }
}

fn possible_digits(value: &str) -> Vec<Digit> {
    match value.len() {
        2 => vec![Digit::One],
        3 => vec![Digit::Seven],
        4 => vec![Digit::Four],
        5 => vec![Digit::Two, Digit::Three, Digit::Five],
        6 => vec![Digit::Six, Digit::Nine, Digit::Zero],
        7 => vec![Digit::Eight],
        _ => vec![],
    }
}

fn contains_chars(some_chars: &[char], v: &str) -> bool {
    v.chars().filter(|c| some_chars.contains(c)).count() == some_chars.len()
}

fn subtract_strings(s1: &str, s2: &str) -> Vec<char> {
    let mut s1_chars: HashSet<char> = s1.chars().collect();
    s2.chars().for_each(|c| {
        s1_chars.remove(&c);
    });
    s1_chars.into_iter().collect()
}

fn find_a(one: &str, seven: &str) -> char {
    let seven_minus_one = subtract_strings(seven, one);
    if seven_minus_one.len() != 1 {
        panic!("cannot find 'a'");
    }
    seven_minus_one[0]
}

fn find_b_d_g(seven: &str, four: &str, five_count: &Vec<String>) -> (char, char, char) {
    let seven_chars: Vec<char> = seven.chars().collect();
    let three = five_count
        .iter()
        .filter(|v| contains_chars(&seven_chars[..], &v))
        .next()
        .unwrap();
    let three_minus_seven = subtract_strings(&three, seven);
    let four_minus_seven = subtract_strings(four, seven);
    // d is the intersection of the 2
    let mut d = '1';
    let mut g = '1';
    let mut b = '1';
    for c in &three_minus_seven {
        if four_minus_seven.contains(&c) {
            d = *c;
        } else {
            g = *c;
        }
    }
    for c in four_minus_seven {
        if three_minus_seven.contains(&c) {
            d = c;
        } else {
            b = c;
        }
    }
    (b, d, g)
}

fn find_c_e_f(b: char, one: &str, seven: &str, five_count: &Vec<String>) -> (char, char, char) {
    let seven_chars: Vec<char> = seven.chars().collect();
    let three = five_count
        .iter()
        .filter(|v| contains_chars(&seven_chars[..], &v))
        .next()
        .unwrap();
    let five = five_count
        .iter()
        .filter(|v| contains_chars(&[b], &v))
        .next()
        .unwrap();
    let mut five_minus_3_minus_one: HashSet<char> = subtract_strings(
        &five,
        &subtract_strings(&three, one)
            .to_vec()
            .iter()
            .collect::<String>(),
    )
    .to_vec()
    .into_iter()
    .collect();
    five_minus_3_minus_one.remove(&b);
    let f = five_minus_3_minus_one.into_iter().next().unwrap();
    let c = subtract_strings(one, &format!("{}", f))[0];
    let two = five_count
        .iter()
        .filter(|v| contains_chars(&[c], &v))
        .filter(|v| !contains_chars(&[f], &v))
        .next()
        .unwrap();
    let e = subtract_strings(&two, &three)[0];
    (c, e, f)
}

fn find_parts(input: Vec<String>) -> Vec<(Digit, HashSet<char>)> {
    let mut digit_by_char_count = Vec::new();
    (0..=8).for_each(|_| digit_by_char_count.push(Vec::new()));
    input.into_iter().for_each(|v| {
        digit_by_char_count[v.len()].push(v);
    });

    let mut parts = ['1'; 7];
    let a = find_a(&digit_by_char_count[2][0], &digit_by_char_count[3][0]);
    parts[0] = a;

    let (b, d, g) = find_b_d_g(
        &digit_by_char_count[3][0],
        &digit_by_char_count[4][0],
        &digit_by_char_count[5],
    );
    parts[1] = b;
    parts[3] = d;
    parts[6] = g;
    let (c, e, f) = find_c_e_f(
        b,
        &digit_by_char_count[2][0],
        &digit_by_char_count[3][0],
        &digit_by_char_count[5],
    );
    parts[2] = c;
    parts[4] = e;
    parts[5] = f;

    let one = [c, f].iter().cloned().collect();
    let two = [a, c, d, e, g].iter().cloned().collect();
    let three = [a, c, d, f, g].iter().cloned().collect();
    let four = [b, c, d, f].iter().cloned().collect();
    let five = [a, b, d, f, g].iter().cloned().collect();
    let six = [a, b, d, e, f, g].iter().cloned().collect();
    let seven = [a, c, f].iter().cloned().collect();
    let eight = [a, b, c, d, e, f, g].iter().cloned().collect();
    let nine = [a, b, c, d, f, g].iter().cloned().collect();
    let zero = [a, b, c, e, f, g].iter().cloned().collect();

    vec![
        (Digit::One, one),
        (Digit::Two, two),
        (Digit::Three, three),
        (Digit::Four, four),
        (Digit::Five, five),
        (Digit::Six, six),
        (Digit::Seven, seven),
        (Digit::Eight, eight),
        (Digit::Nine, nine),
        (Digit::Zero, zero),
    ]
}

fn read_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let parse_line = |line: &str| {
        let mut parts = line.split(" | ");
        let inputs: Vec<String> = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|v| v.to_owned())
            .collect();
        let outputs: Vec<String> = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|v| v.to_owned())
            .collect();
        (inputs, outputs)
    };

    input.lines().map(parse_line).collect()
}

fn solve_single(input: Vec<String>, output: Vec<String>) -> (usize, u64) {
    let p1 = output
        .iter()
        .filter(|s| possible_digits(s).len() == 1)
        .count();
    let mut all: Vec<String> = input.clone();
    all.extend_from_slice(&output);

    let digits = find_parts(all);
    let p2 = output
        .iter()
        // get the set of chars that make up each digit
        .map(|s| s.chars().collect::<HashSet<char>>())
        .map(|digit_set| {
            &digits
                .iter()
                .find(|d| d.1 == digit_set)
                .expect(&format!("not able to find something for {:?}", digit_set))
                .0
        })
        .fold(0, |curr, digit| (curr * 10) + digit.as_u64());

    (p1, p2)
}

fn solve(input: Vec<(Vec<String>, Vec<String>)>) -> (usize, u64) {
    input
        .into_iter()
        .map(|line| solve_single(line.0, line.1))
        .fold((0, 0), |(acc_p1, acc_p2), (p1, p2)| {
            (acc_p1 + p1, acc_p2 + p2)
        })
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let input = read_input(&read_to_string(file_path).unwrap());
    let (p1, p2) = solve(input);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
