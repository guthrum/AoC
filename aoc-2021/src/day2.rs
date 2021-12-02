use std::{fs::read_to_string, num::ParseIntError};

fn solve(input: &str) -> (i64, i64) {
    let instructions: Vec<(&str, i64)> = input
        .lines()
        .map(|line| {
            let mut l = line.split(" ");
            (l.next().unwrap(), l.next().unwrap().parse::<i64>().unwrap())
        })
        .collect();
    let (horiz, vert) = instructions
        .iter()
        .fold((0, 0), |(h, v), (instruc, amount)| {
            match instruc.chars().next().unwrap() {
                'f' => (h + amount, v),
                'd' => (h, v + amount),
                'u' => (h, v - amount),
                _ => panic!("ahhh"),
            }
        });
    let (horiz2, vert2, aim) =
        instructions
            .iter()
            .fold((0, 0, 0), |(h, v, aim), (instruc, amount)| {
                match instruc.chars().next().unwrap() {
                    'f' => (h + amount, v + (aim * amount), aim),
                    'd' => (h, v, aim + amount),
                    'u' => (h, v, aim - amount),
                    _ => panic!("ahhh"),
                }
            });
    (horiz * vert, horiz2 * vert2)
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
