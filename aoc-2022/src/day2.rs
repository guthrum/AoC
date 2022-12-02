use std::fs::read_to_string;

fn score_1(line: &str) -> i32 {
    match line {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => {
            panic!("invalid line {}", line)
        }
    }
}

fn score_2(line: &str) -> i32 {
    match line {
        "A X" => 3 + 0,
        "A Y" => 1 + 3,
        "A Z" => 2 + 6,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 2 + 0,
        "C Y" => 3 + 3,
        "C Z" => 1 + 6,
        _ => {
            panic!("invalid line {}", line)
        }
    }
}

fn solve(input: &str) -> (i32, i32) {
    let values: Vec<&str> = input.lines().collect();
    let part1 = values.iter().map(|line| score_1(*line)).sum();
    let part2 = values.iter().map(|line| score_2(*line)).sum();

    (part1, part2)
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
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(solve(input), (15, 12));
    }
}
