use std::fs::read_to_string;

fn solve_p1(input: &str) -> i32 {
    let positions: Vec<i32> = input.lines().next().unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut cost = i32::MAX;
    for pos in min..=max {
        let pos_cost = positions.iter()
            .map(|v| (pos-v).abs())
            .sum();
        cost = i32::min(cost, pos_cost);
    }

    cost
}

fn solve_p2(input: &str) -> i32 {
    let positions: Vec<i32> = input.lines().next().unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut cost = i32::MAX;
    for pos in min..=max {
        let pos_cost = positions.iter()
            .map(|v| (pos-v).abs())
            .filter(|v| *v != 0)
            .map(|v| (v*(v+1))/2)
            .sum();
        cost = i32::min(cost, pos_cost);
    }

    cost
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let input = read_to_string(file_path).unwrap();
    println!("Part 1 = {}", solve_p1(&input));
    println!("Part 2 = {}", solve_p2(&input));
}
