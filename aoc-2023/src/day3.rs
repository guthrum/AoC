use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::Hash;

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point(usize, usize);

#[derive(Debug, Clone)]
struct PartNumber {
    number: u32,
    symbols: HashSet<(char, Point)>,
}

fn get_neighbours(
    matrix: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Vec<(char, Point)> {
    // y then x
    let mut res = vec![];
    for dy in vec![-1, 0, 1] {
        for dx in vec![-1, 0, 1] {
            let nx = x + dx;
            let ny = y + dy;
            if nx > 0 && nx < width && ny > 0 && ny < height {
                let c = *matrix.get(ny as usize).unwrap().get(nx as usize).unwrap();
                if is_symbol(c) {
                    res.push((c, Point(nx as usize, ny as usize)));
                }
            }
        }
    }
    res
}

fn parse_problem(input: &str) -> (Vec<PartNumber>, HashMap<Point, Vec<PartNumber>>) {
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let width = matrix.get(0).unwrap().len();
    let height = matrix.len();

    let mut part_numbers = Vec::new();
    let mut gear_locations: HashMap<Point, Vec<PartNumber>> = HashMap::new();

    for (y, row) in matrix.iter().enumerate() {
        let mut x = 0;
        while x < row.len() {
            let current = row.get(x).unwrap();
            if current.is_digit(10) {
                let mut c = *current;
                // consume number
                let mut digits = String::new();
                let mut symbols = HashSet::new();
                while x < width && c.is_digit(10) {
                    digits.push(c);
                    // add the neighbours
                    get_neighbours(&matrix, x as i32, y as i32, width as i32, height as i32)
                        .into_iter()
                        .for_each(|c| {
                            symbols.insert(c);
                        });
                    x += 1;
                    c = *row.get(x).unwrap_or(&'.');
                }
                let number = u32::from_str_radix(&digits, 10)
                    .expect(&format!("failed to parse number {digits}"));
                let part_number = PartNumber { symbols, number };
                part_number
                    .symbols
                    .iter()
                    .filter(|(c, _)| *c == '*')
                    .for_each(|(_, point)| {
                        gear_locations
                            .entry(point.clone())
                            .or_default()
                            .push(part_number.clone())
                    });

                part_numbers.push(part_number)
            } else {
                x += 1;
            }
        }
    }
    (part_numbers, gear_locations)
}

fn solve(input: &str) -> (u32, u32) {
    let (part_numbers, gear_locations) = parse_problem(input);
    let p1 = part_numbers
        .iter()
        .filter(|pn| !pn.symbols.is_empty())
        .map(|pn| pn.number)
        .sum();
    let p2 = gear_locations
        .iter()
        .filter(|(_, pn)| pn.len() == 2)
        .map(|(_, pns)| pns.iter().map(|pn| pn.number).product::<u32>())
        .sum();

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
    use super::solve;

    #[test]
    fn example_input() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(solve(input), (4361, 0));
    }
}
