use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Cell {
    value: i8,
    max_north: i8,
    max_east: i8,
    max_south: i8,
    max_west: i8,
}

impl Cell {
    fn visible(&self) -> bool {
        self.max_north < self.value
            || self.max_south < self.value
            || self.max_east < self.value
            || self.max_west < self.value
    }
}

impl From<char> for Cell {
    fn from(char_value: char) -> Self {
        Self {
            value: char_value.to_digit(10).unwrap() as i8,
            max_north: i8::MAX,
            max_east: i8::MAX,
            max_south: i8::MAX,
            max_west: i8::MAX,
        }
    }
}

fn read_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect()
}

fn solve_1(mut input: Vec<Vec<Cell>>) -> usize {
    // preprocess
    // north
    for s in 0..input.get(0).unwrap().len() {
        let mut max = -1;
        for f in 0..input.len() {
            let mut cell = input.get_mut(f).unwrap().get_mut(s).unwrap();
            cell.max_north = max;
            max = max.max(cell.value);
        }
    }

    // east
    for line in input.iter_mut() {
        let mut max = -1;
        for mut cell in line.iter_mut().rev() {
            cell.max_east = max;
            max = max.max(cell.value);
        }
    }

    // south
    for s in 0..input.get(0).unwrap().len() {
        let mut max = -1;
        for f in 0..input.len() {
            let idx = input.len() - f - 1;
            let mut cell = input.get_mut(idx).unwrap().get_mut(s).unwrap();
            cell.max_south = max;
            max = max.max(cell.value);
        }
    }

    // west
    for line in input.iter_mut() {
        let mut max = -1;
        for mut cell in line.iter_mut() {
            cell.max_west = max;
            max = max.max(cell.value);
        }
    }

    input.iter().flatten().filter(|c| c.visible()).count()
}

fn solve(input_str: &str) -> (usize, usize) {
    let input = read_input(input_str);

    (solve_1(input), 0)
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
        let input = r#"30373
25512
65332
33549
35390"#;
        assert_eq!(solve(input), (21, 0));
    }
}
