use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut res = Vec::with_capacity(50);
    for line in input.lines() {
        let mut split = line.split(" (contains ");
        let ingrediants = split
            .next()
            .unwrap()
            .split(" ")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let contains = split.next().unwrap().replace(")", "").split(", ")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        res.push((ingrediants, contains));
    }

    res
}

fn solve(input: Vec<(Vec<String>, Vec<String>)>) -> (usize, usize) {
    (0, 0)
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let (p1, p2) = solve(parse_input(&read_to_string(file_path).unwrap()));
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
