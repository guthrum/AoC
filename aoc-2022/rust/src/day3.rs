use std::collections::HashSet;
use std::fs::read_to_string;

fn read_line(line: &str) -> (HashSet<char>, HashSet<char>) {
    assert_eq!(line.len() % 2, 0);
    let first = line.chars().take(line.len() / 2).collect();
    let second = line.chars().skip(line.len() / 2).collect();
    (first, second)
}

fn value(c: char) -> u32 {
    if c.is_uppercase() {
        27 + (c as u32) - ('A' as u32)
    } else {
        1 + (c as u32) - ('a' as u32)
    }
}

fn part_1(input: &Vec<(HashSet<char>, HashSet<char>)>) -> u32 {
    input
        .iter()
        .map(|(first, second)| *first.intersection(second).next().unwrap())
        .map(value)
        .sum()
}

fn union(sets: (HashSet<char>, HashSet<char>)) -> HashSet<char> {
    let (mut s1, s2) = sets;
    for x in s2 {
        s1.insert(x);
    }
    s1
}

fn part_2(input: Vec<(HashSet<char>, HashSet<char>)>) -> u32 {
    let mut iter = input.into_iter().peekable();
    let mut sum = 0;
    while iter.peek().is_some() {
        let s1 = union(iter.next().unwrap());
        let s2 = union(iter.next().unwrap());
        let p1: HashSet<char> = s1.intersection(&s2).cloned().collect();
        let s3 = union(iter.next().unwrap());
        let common = *p1.intersection(&s3).next().unwrap();
        sum += value(common);
    }
    sum
}

fn solve(input: &str) -> (u32, u32) {
    let values: Vec<(HashSet<char>, HashSet<char>)> = input.lines().map(read_line).collect();
    let p1 = part_1(&values);
    let p2 = part_2(values);

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
    use crate::solve;

    #[test]
    fn example_input() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(solve(input), (157, 70));
    }
}
