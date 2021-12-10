use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn read_input(path: &str) -> Vec<Vec<u8>> {
    let mut res = Vec::with_capacity(100);
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| u8::from_str_radix(&c.to_string(), 10).unwrap())
                .collect::<Vec<u8>>()
        })
        .for_each(|v| {
            res.push(v);
        });

    res
}

fn solve(input: &Vec<Vec<u8>>) -> (u32, usize) {
    let mut local_minimas = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let neighbours_count = vec![
                (x as i32 + 1, y as i32),
                (x as i32 - 1, y as i32),
                (x as i32, y as i32 + 1),
                (x as i32, y as i32 - 1),
            ]
            .into_iter()
            .filter(|(px, py)| *px >= 0 && *py >= 0)
            .map(|(px, py)| {
                input
                    .get(py as usize)
                    .map(|row| *row.get(px as usize).unwrap_or(&10))
                    .unwrap_or(10)
            })
            .filter(|v| v <= value)
            .count();
            if neighbours_count == 0 {
                local_minimas.push((x, y, value));
            }
        }
    }
    let p1 = local_minimas.iter().map(|v| *v.2 as u32 + 1).sum();
    let mut basin_count = 0;
    let mut basins: HashMap<i64, usize> = HashMap::new();

    let mut done = HashSet::with_capacity(10000);

    for (px, py, _) in local_minimas {
        basin_count += 1;
        let mut queue = Vec::new();
        done.clear();
        queue.push((px, py));
        while let Some(next) = queue.pop() {
            done.insert(next);

            let (x, y) = next;
            let neighbours: Vec<(usize, usize)> = vec![
                (x as i32 + 1, y as i32),
                (x as i32 - 1, y as i32),
                (x as i32, y as i32 + 1),
                (x as i32, y as i32 - 1),
            ]
            .into_iter()
            .filter(|(px, py)| *px >= 0 && *py >= 0)
            .map(|(px, py)| {
                (
                    px,
                    py,
                    input
                        .get(py as usize)
                        .map(|row| *row.get(px as usize).unwrap_or(&9))
                        .unwrap_or(9),
                )
            })
            .filter(|v| v.2 < 9)
            .map(|v| (v.0 as usize, v.1 as usize))
            .filter(|v| !done.contains(v))
            .collect();
            queue.extend_from_slice(neighbours.as_slice());
        }
        basins.insert(basin_count, done.len());
    }
    let mut basin_sizes: Vec<usize> = basins.values().cloned().collect();
    basin_sizes.sort();
    basin_sizes.reverse();
    let p2 = basin_sizes[0..=2].iter().product();

    (p1, p2)
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let input = read_input(&file_path);
    let (p1, p2) = solve(&input);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
