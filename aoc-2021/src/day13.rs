use std::{
    collections::{HashSet},
    fs::read_to_string,
};

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn read_input(path: &str) -> (HashSet<(i32, i32)>, Vec<Fold>) {
    let raw = read_to_string(path).unwrap();
    let mut lines = raw.lines();
    let mut coords = HashSet::new();
    for raw_coord in lines.by_ref() {
        if raw_coord.is_empty() {
            break;
        }
        let mut coord = raw_coord.split(',');
        let x = i32::from_str_radix(coord.next().unwrap(), 10).unwrap();
        let y = i32::from_str_radix(coord.next().unwrap(), 10).unwrap();
        coords.insert((x, y));
    }

    let mut folds = Vec::new();
    for raw_fold in lines {
        let raw = raw_fold.replace("fold along ", "");
        let value = i32::from_str_radix(raw.split('=').nth(1).unwrap(), 10).unwrap();
        let fold = if raw.starts_with('x') {
            Fold::X(value)
        } else if raw.starts_with('y') {
            Fold::Y(value)
        } else {
            panic!("cannot handle {}", raw);
        };
        folds.push(fold);
    }

    (coords, folds)
}

fn solve(input: (HashSet<(i32, i32)>, Vec<Fold>)) -> (usize, usize) {
    let (mut coords, folds) = input;
    let mut p1 = None;

    for fold in folds {
        let mut new_coords = HashSet::with_capacity(coords.len());
        for coord in coords {
            let new_coord = match fold {
                Fold::X(fx) => {
                    let new_x = if coord.0 <= fx {
                        coord.0
                    } else {
                        (2 * fx) - coord.0
                    };
                    (new_x, coord.1)
                }
                Fold::Y(fy) => {
                    let new_y = if coord.1 <= fy {
                        coord.1
                    } else {
                        (2 * fy) - coord.1
                    };
                    (coord.0, new_y)
                }
            };
            if new_coord.0 < 0 || new_coord.1 < 0 {
                panic!("something went wrong");
            }
            new_coords.insert(new_coord);
        }
        if p1.is_none() {
            p1 = Some(new_coords.len());
        }

        coords = new_coords;
    }
    let xmax = coords.iter().map(|c| c.0).max().unwrap();
    let ymax = coords.iter().map(|c| c.1).max().unwrap();
    for y in 0..=ymax {
        for x in 0..=xmax {
            let c = if coords.contains(&(x, y)) { "#" } else { "." };
            print!("{}", c);
        }
        println!();
    }

    (p1.unwrap(), coords.len())
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let input = read_input(&file_path);
    let (p1, p2) = solve(input);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
