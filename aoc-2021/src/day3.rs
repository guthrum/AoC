use std::fs::read_to_string;

fn get_high_low(values: &Vec<Vec<usize>>, idx: usize) -> (usize, usize) {
    let [c0, c1] = values.iter().map(|v| v[idx]).fold([0, 0], |mut acc, v| {
        acc[v] += 1;
        acc
    });
    if c0 > c1 {
        (0, 1)
    } else {
        (1, 0)
    }
}

fn solve(input: &str) -> (usize, usize) {
    let values: Vec<Vec<usize>> = input
        .lines()
        .map(|v| {
            v.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let length = values[0].len();
    let mut counts = vec![[0, 0]; length];
    for line in &values {
        for (idx, d) in line.iter().enumerate() {
            counts[idx][*d] += 1;
        }
    }

    let high_low: Vec<(usize, usize)> = counts
        .iter()
        .map(|v| if v[0] > v[1] { (0, 1) } else { (1, 0) })
        .collect();
    let (high_p1, low_p1) = high_low
        .into_iter()
        .fold((0, 0), |(h, l), v| ((h << 1) + v.0, (l << 1) + v.1));

    let mut high_values = values.clone();
    let mut low_values = values;

    for idx in 0..length {
        if high_values.len() != 1 {
            let target = get_high_low(&high_values, idx).0;
            high_values = high_values.into_iter().partition(|v| v[idx] == target).0;
        }
        if low_values.len() != 1 {
            let target = get_high_low(&low_values, idx).1;
            low_values = low_values.into_iter().partition(|v| v[idx] == target).0;
        }
    }
    let (high_p2, low_p2) = high_values[0]
        .iter()
        .zip(low_values[0].iter())
        .fold((0, 0), |(h, l), v| ((h << 1) + v.0, (l << 1) + v.1));

    (low_p1 * high_p1, low_p2 * high_p2)
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
