use std::fs::read_to_string;

fn solve(input: &str) -> (usize, usize) {
    let values: Vec<i32> = input
        .lines()
        .map(|v| i32::from_str_radix(v, 10).unwrap())
        .collect();
    let part1 = values
        .iter()
        .skip(1)
        .zip(values.iter())
        .map(|(curr, prev)| curr - prev)
        .filter(|v| *v > 0)
        .count();
    let part2 = values
        .iter()
        .skip(3)
        .zip(values.iter())
        .map(|(curr, prev)| curr - prev)
        .filter(|v| *v > 0)
        .count();
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
        let input = r#"199
200
208
210
200
207
240
269
260
263"#;
        assert_eq!(solve(input), (7, 5));
    }
}
