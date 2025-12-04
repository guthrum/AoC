type Line = Vec<u8>;

fn read_input(contents: String) -> Vec<Line> {
    contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("invalid digit"))
                .map(|d| d as u8)
                .collect()
        })
        .collect()
}

fn part1(lines: &[Line]) -> u64 {
    let largest_joltage = |line: &Line| {
        let mut first_digit = 0;
        let last_digit = line.last().unwrap();
        let mut second_digit = last_digit;
        for d in &line[..line.len() - 1] {
            if *d > first_digit {
                first_digit = *d;
                second_digit = last_digit
            } else if d > second_digit {
                second_digit = d;
            }
        }
        (first_digit * 10 + second_digit) as u64
    };
    lines.iter().map(largest_joltage).sum()
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("missing file as first argument");
    let contents = std::fs::read_to_string(file).expect("failed to read file");
    let input = read_input(contents);
    println!("Part 1 = {}", part1(&input));
}
