use std::fs::read_to_string;

#[derive(Debug)]
enum Operation {
    Noop,
    NoopAddX,
    AddX(i64),
}

impl Operation {
    fn delta_x(&self) -> i64 {
        match *self {
            Self::AddX(i) => i,
            Self::Noop => 0,
            Self::NoopAddX => 0,
        }
    }
}

fn read_input(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            if line == "noop" {
                vec![Operation::Noop]
            } else if let Some(amount) = line.strip_prefix("addx ") {
                let amount = i64::from_str_radix(amount, 10).unwrap();
                vec![Operation::NoopAddX, Operation::AddX(amount)]
            } else {
                panic!();
            }
        })
        .flatten()
        .collect()
}

fn part_1(operations: &[Operation]) -> i64 {
    let mut instructions = operations.iter();
    let mut x = 1;
    let mut sum_strength = 0;
    for cycle in 1..=220 {
        let instruction = instructions.next().unwrap();
        if vec![20, 60, 100, 140, 180, 220].contains(&cycle) {
            sum_strength += cycle * x;
            println!(
                "\tcycle = {} x = {} strength = {} sum = {} instruction = {:?}",
                cycle,
                x,
                cycle * x,
                sum_strength,
                instruction
            );
        }
        x += instruction.delta_x();
    }

    sum_strength
}

fn solve(lines: &str) -> (i64, i64) {
    let input = read_input(lines);
    (part_1(&input), 0)
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
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
        assert_eq!(solve(input), (13140, 0));
    }
}
