use std::fs::read_to_string;

fn parse_match(m: &str) -> &str {
    match m {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        m => m,
    }
}

fn calibration_value(line: &str, valid_digits: &Vec<&str>) -> u32 {
    let mut digits: Vec<&str> = Vec::new();
    for idx in 0..line.len() {
        for valid_digit in valid_digits {
            let ul = line.len().min(idx + valid_digit.len());
            let ss = &line[idx..ul];
            if ss == *valid_digit {
                digits.push(ss);
            }
        }
    }

    let d1 = digits
        .first()
        .map(|m| parse_match(*m))
        .expect("missing first number");
    let d2 = digits
        .last()
        .map(|m| parse_match(*m))
        .expect("missing second number");
    u32::from_str_radix(&format!("{}{}", d1, d2), 10).expect("failed to parse u32")
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solve(input: &str) -> (u32, u32) {
    let part1 = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let part2 = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let lines = parse_input(input);
    let res1 = lines
        .iter()
        .map(|line| calibration_value(line, &part1))
        .sum();
    let res2 = lines
        .iter()
        .map(|line| calibration_value(line, &part2))
        .sum();

    (res1, res2)
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
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(solve(input), (142, 142));
    }
}
