use std::fs::read_to_string;

struct Range {
    start: u32,
    end: u32,
}

impl From<&str> for Range {
    fn from(raw: &str) -> Self {
        let mut ends = raw.split('-');
        let start = u32::from_str_radix(ends.next().unwrap(), 10).unwrap();
        let end = u32::from_str_radix(ends.next().unwrap(), 10).unwrap();
        Self { start, end }
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlap(&self, other: &Range) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
    }
}

fn read_line(raw: &str) -> (Range, Range) {
    let mut ranges = raw.split(',');
    let f = Range::from(ranges.next().unwrap());
    let s = Range::from(ranges.next().unwrap());
    (f, s)
}

fn part_1(input: &Vec<(Range, Range)>) -> usize {
    input
        .iter()
        .filter(|(f, s)| f.contains(s) || s.contains(f))
        .count()
}

fn part_2(input: &Vec<(Range, Range)>) -> usize {
    input
        .iter()
        .filter(|(f, s)| f.overlap(s) || s.overlap(f))
        .count()
}

fn solve(input: &str) -> (usize, usize) {
    let values: Vec<(Range, Range)> = input.lines().map(read_line).collect();
    let p1 = part_1(&values);
    let p2 = part_2(&values);

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
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        assert_eq!(solve(input), (2, 4));
    }
}
