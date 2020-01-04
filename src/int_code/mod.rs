extern crate termion;

use std::fs;
use std::io::{self};

pub mod machine;
pub mod monitor;

pub fn read_file(path: &str) -> io::Result<Vec<i64>> {
    Ok(fs::read_to_string(path)?
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect())
}
