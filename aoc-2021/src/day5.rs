use std::{collections::HashMap, fs::read_to_string};

fn read_coords(p: &str) -> (i32, i32) {
    let mut parts = p.split(",");
    (
        i32::from_str_radix(parts.next().unwrap(), 10).unwrap(),
        i32::from_str_radix(parts.next().unwrap(), 10).unwrap(),
    )
}

fn parse(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let mut res = Vec::with_capacity(1000);
    res.extend(input
        .lines()
        .map(|l| {
            let mut coords = l.split(" -> ");
            (read_coords(coords.next().unwrap()), read_coords(coords.next().unwrap()))
        }));
    res
}

fn points_on_line(p1: (i32, i32), p2: (i32, i32)) -> Box<dyn Iterator<Item = (i32, i32)>> {
    if p1.0 == p2.0 {
        if p1.1 > p2.1 {
            points_on_line(p2, p1)
        } else {
            Box::new((p1.1..=p2.1).into_iter().map(move |y| (p1.0, y)))
        }
    } else if p1.1 == p2.1 {
        if p1.0 > p2.0 {
            points_on_line(p2, p1)
        } else {
            Box::new((p1.0..=p2.0).into_iter().map(move |x| (x, p1.1)))
        }
    } else {
        let dx = if p1.0 < p2.0 { 1 } else { -1 };
        let dy = if p1.1 < p2.1 { 1 } else { -1 };
        let mut res = Vec::with_capacity((p1.0 - p2.0).abs() as usize);
        let mut p = p1;
        res.push(p);
        while p != p2 {
            p = (p.0 + dx, p.1 + dy);
            res.push(p)
        }
        Box::new(res.into_iter())
    }
}

fn solve(input: &str) -> (usize, usize) {
    let line_sections = parse(input);
    let mut line_sections_part_1: HashMap<(i32, i32), i32> = HashMap::new();
    line_sections
        .iter()
        .filter(|v| v.0 .0 == v.1 .0 || v.0 .1 == v.1 .1)
        .cloned()
        .flat_map(|(p1, p2)| points_on_line(p1, p2).into_iter())
        .for_each(|p| {
            let count = line_sections_part_1.entry(p).or_insert(0);
            *count += 1;
        });
    let res1 = line_sections_part_1.values().filter(|v| **v > 1).count();

    let mut line_sections_part_2: HashMap<(i32, i32), i32> = HashMap::new();
    line_sections
        .into_iter()
        .flat_map(|(p1, p2)| points_on_line(p1, p2).into_iter())
        .for_each(|p| {
            let count = line_sections_part_2.entry(p).or_insert(0);
            *count += 1;
        });
    let res2 = line_sections_part_2.values().filter(|v| **v > 1).count();
    (res1, res2)
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
