use std::collections::HashSet;
use std::fs::read_to_string;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta_move(&self) -> (i32, i32) {
        match *self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Position(i32, i32);

impl Position {
    fn add(&mut self, delta: (i32, i32)) {
        self.0 += delta.0;
        self.1 += delta.1;
    }

    fn within_1(&self, other: &Position) -> bool {
        let dx = (self.0 - other.0).abs();
        let dy = (self.1 - other.1).abs();
        dx <= 1 && dy <= 1
    }

    fn move_to_within_1(&mut self, other: &Position) {
        assert!(!self.within_1(other));
        let dx = other.0 - self.0;
        let dy = other.1 - self.1;
        self.0 += dx.signum();
        self.1 += dy.signum();
    }
}

fn read_input(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|line| {
            let direction = match &line[0..=0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!(),
            };
            let amount = i32::from_str_radix(&line[2..], 10).unwrap();
            (direction, amount)
        })
        .collect()
}

fn solve_for(input: &[(Direction, i32)], count: usize) -> usize {
    let mut knots = vec![Position(0, 0); count];

    let mut tails = HashSet::new();
    tails.insert(knots.last().unwrap().clone());
    for (direction, amount) in input {
        for _ in 0..*amount {
            knots.first_mut().unwrap().add(direction.delta_move());

            for i in 0..knots.len() - 1 {
                let previous = knots.get(i).unwrap().clone();
                if !knots.get(i + 1).unwrap().within_1(&previous) {
                    knots.get_mut(i + 1).unwrap().move_to_within_1(&previous);
                }
            }

            tails.insert(knots.last().unwrap().clone());
        }
    }
    tails.len()
}

fn solve(lines: &str) -> (usize, usize) {
    let input = read_input(lines);
    (solve_for(&input, 2), solve_for(&input, 10))
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
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        assert_eq!(solve(input), (13, 1));
    }
}
