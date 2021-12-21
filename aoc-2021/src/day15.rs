use std::{fs::read_to_string};

fn read_input(path: &str) -> Vec<Vec<u8>> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| u8::from_str_radix(&c.to_string(), 10).unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Greater,
    Less,
    Na,
}

fn create_part_2(input: &Vec<Vec<u8>>, multiplier: usize) -> Vec<Vec<u8>> {
    let mut new_grid = Vec::with_capacity(input.len() * multiplier);
    for _ in 1..=input.len() * multiplier {
        new_grid.push(vec![0; input[0].len() * multiplier]);
    }
    for my in 0..multiplier {
        for mx in 0..multiplier {
            for oy in 0..input.len() {
                for ox in 0..input[0].len() {
                    let x = mx * input[0].len() + ox;
                    let y = my * input[0].len() + oy;
                    let value = 1 + ((input[oy][ox] as usize + my + mx - 1) % 9);
                    new_grid[y][x] = value as u8;
                }
            }
        }
    }

    new_grid
}

fn solve_single(input: Vec<Vec<u8>>) -> u32 {
    // the minimum risk path (total cost, direction of travel into cell)
    let mut min_risk_path = Vec::with_capacity(input.len());
    for r in &input {
        min_risk_path.push(vec![(u32::MAX, Direction::Na); r.len()]);
    }
    min_risk_path[0][0] = (0, Direction::Na);

    // loop through each cell to update its shortest path
    let mut changed = true;
    while changed {
        let mut new_min_cost = min_risk_path.clone();
        changed = false;
        for x in 0..input.len() {
            for y in 0..input[0].len() {
                // we just ignore the start position from this
                if !(x == 0 && y == 0) {
                    let (min_neighbour, direction) = vec![
                        (x as i32 + 1, y as i32, Direction::Right),
                        (x as i32 - 1, y as i32, Direction::Left),
                        (x as i32, y as i32 + 1, Direction::Greater),
                        (x as i32, y as i32 - 1, Direction::Less),
                    ]
                    .into_iter()
                    .filter(|(px, py, _)| *px >= 0 && *py >= 0)
                    .filter(|(px, py, _)| {
                        (*px as usize) < input.len() && (*py as usize) < input[0].len()
                    })
                    .map(|(px, py, dir)| (min_risk_path[px as usize][py as usize].0, dir))
                    .min_by(|l, r| {
                        if l.0 < r.0 {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    })
                    .unwrap();
                    if min_neighbour < u32::MAX {
                        let min_cost = input[x][y] as u32 + min_neighbour;
                        if min_cost < min_risk_path[x][y].0 {
                            changed = true;
                            new_min_cost[x][y] = (min_cost, direction);
                        }
                    } else {
                        changed = true;
                    }
                }
            }
        }
        min_risk_path = new_min_cost;
    }

    min_risk_path[input.len() - 1][input[0].len() - 1].0
}

fn print_grid(grid: &Vec<Vec<u8>>) {
    for row in grid {
        for v in row {
            print!("{}", v);
        }
        println!();
    }
}

fn solve(input: Vec<Vec<u8>>) -> (u32, u32) {
    let larger = create_part_2(&input, 5);
    (solve_single(input), solve_single(larger))
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let input = read_input(&file_path);
    let (p1, p2) = solve(input);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
