use std::collections::HashSet;
use std::fs::read_to_string;

fn solve_for(input: &str, offset: usize) -> usize {
    for i in 0..(input.len() - offset - 1) {
        let window: HashSet<char> = input[i..i + offset].chars().into_iter().collect();
        if window.len() == offset {
            return i + offset;
        }
    }
    0
}

fn solve(input: &str) -> (usize, usize) {
    (solve_for(input, 4), solve_for(input, 14))
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
        let input = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;
        assert_eq!(solve(input), (7, 19));
    }

    #[test]
    fn example_input2() {
        let input = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#;
        assert_eq!(solve(input), (5, 23));
    }
}
