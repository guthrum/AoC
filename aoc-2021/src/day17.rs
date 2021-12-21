use std::{fs::read_to_string};

fn read_input(path: &str) -> ((i32, i32), (i32, i32)) {
    let input = read_to_string(path)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .replace("target area: ", "")
        ;
    let mut target_area = input.split(", ");
    let x_range: Vec<i32> = target_area
        .next()
        .unwrap()
        .replace("x=", "")
        .split("..")
        .map(|s| i32::from_str_radix(s, 10).unwrap_or_else(|_| panic!("failed to parse {}", s)))
        .collect();
    let y_range: Vec<i32> = target_area
        .next()
        .unwrap()
        .replace("y=", "")
        .split("..")
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .collect();
    ((x_range[0], x_range[1]), (y_range[0], y_range[1]))
}

fn valid(mut dx: i32, mut dy: i32, target: &((i32, i32), (i32, i32))) -> Vec<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    let mut points = Vec::new();
    loop {
        points.push((x, y));
        if target.0 .0 <= x && x <= target.0 .1 && target.1 .0 <= y && y <= target.1 .1 {
            return points;
        }
        x += dx;
        y += dy;
        dx = i32::max(0, dx - 1);
        dy -= 1;

        if (x == 0 && x < target.0 .0) || target.0 .1 < x || y < target.1 .0 {
            return Vec::new();
        }
    }
}

fn solve(input: ((i32, i32), (i32, i32))) -> (i32, usize) {
    let paths = (0..=(input.0 .1))
        .flat_map(|x| {
            (input.1 .0..=input.1 .0.abs())
                .map(|y| ((x, y), valid(x, y, &input)))
                .filter(|v| !v.1.is_empty())
                .collect::<Vec<((i32, i32), Vec<(i32, i32)>)>>()
        })
        .collect::<Vec<((i32, i32), Vec<(i32, i32)>)>>();

    let p1 = paths
        .iter()
        .map(|(_, path)| path)
        .map(|path| path.iter().map(|(_, y)| *y).max().unwrap())
        .max()
        .unwrap();

    (p1, paths.len())
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let input = read_input(&file_path);
    let (p1, p2) = solve(input);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
