use std::fs::read_to_string;

fn solve(input: &str) -> (i32, i32) {
    let mut values: Vec<i32> = input
        .split("\n\n")
        .map(|v| {
            v.split('\n')
                .filter(|v| !v.is_empty())
                .map(|v| i32::from_str_radix(v, 10).expect(&format!("{} is not i32", v)))
                .sum()
        })
        .collect();
    values.sort_by(|a, b| b.cmp(a));

    let sum_first_3 = values.iter().take(3).sum();

    (*values.first().unwrap(), sum_first_3)
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
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        assert_eq!(solve(input), (24000, 45000));
    }
}
