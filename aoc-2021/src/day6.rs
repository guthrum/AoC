use std::fs::read_to_string;

fn solve(input: &str, days: usize) -> usize {
    let mut fish_life_span_counts = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for v in input.lines().next().unwrap().split(",") {
        let idx = usize::from_str_radix(v, 10).unwrap();
        fish_life_span_counts[idx] += 1;
    }

    for _day in 0..days {
        let day_0 = fish_life_span_counts.remove(0);
        fish_life_span_counts[6] += day_0;
        fish_life_span_counts.push(day_0);
    }

    fish_life_span_counts.iter().sum()
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let contents = read_to_string(file_path).unwrap();
    println!("Part 1 = {}", solve(&contents, 80));
    println!("Part 2 = {}", solve(&contents, 256));
}
