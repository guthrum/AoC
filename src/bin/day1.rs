use std::fs::File;
use std::io::{BufReader, BufRead};

fn calculate_fuel(mass: &f64) -> f64 {
    let fuel = (mass / 3_f64).floor() - 2_f64;
    if fuel <= 0_f64 {
        return 0_f64;
    }
    fuel + calculate_fuel(&fuel)
}

fn read_file(path: &str) -> std::io::Result<Vec<f64>> {
    let file = File::open(path)?;
    let buffer = BufReader::new(&file);
    let numbers: Vec<f64> = buffer.lines()
        .map(|s| s.unwrap())
        .map(|s| s.parse().unwrap())
        .collect();
    Ok(numbers)
}

fn main() {
    let numbers = read_file("/home/tim/projects/AoC19/resources/day1input")
        .expect("unable to load numbers");
    let fuel: f64 = numbers.iter()
        .map(|n| calculate_fuel(n))
        .sum();
    println!("{:?}", fuel);
}
