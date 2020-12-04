use structopt::StructOpt;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn valid(passport: &HashMap<String, String>) -> bool {
    let valid = (passport.contains_key("cid") && passport.len() == 8)
        || (!passport.contains_key("cid") && passport.len() == 7);
    valid
}

fn valid_hgt(hgt: &str) -> bool {
    if hgt.ends_with("cm") {
        let cm: i32 = hgt.strip_suffix("cm").unwrap().parse().unwrap_or(0);
        return 150 <= cm && cm <= 193;
    }
    if hgt.ends_with("in") {
        let inchs: i32 = hgt.strip_suffix("in").unwrap().parse().unwrap_or(0);
        return 59 <= inchs && inchs <= 76;
    }
    false
}

fn valid_hair(hair: &str) -> bool {
    if !hair.starts_with("#") || hair.len() != 7 {
        return false;
    }
    let hair_hex = hair.strip_prefix("#").unwrap();
    if hair_hex.to_lowercase() != hair_hex {
        return false;
    }
    hair_hex.chars().filter(|c| !c.is_digit(16)).count() == 0
}

fn valid_eye(eye: &str) -> bool {
    match eye {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn valid_passport(pid: &str) -> bool {
    if pid.len() != 9 {
        return false;
    }
    pid.chars().filter(|c| !c.is_digit(10)).count() == 0
}

fn security_checks(passport: &HashMap<String, String>) -> bool {
    if !valid(passport) {
        return false;
    }

    let byr: i32 = passport.get("byr").unwrap().parse().unwrap();
    let iyr: i32 = passport.get("iyr").unwrap().parse().unwrap();
    let eyr: i32 = passport.get("eyr").unwrap().parse().unwrap();
    let hgt_raw = passport.get("hgt").unwrap();
    let hcl_raw = passport.get("hcl").unwrap();
    let eye = passport.get("ecl").unwrap();
    let pid = passport.get("pid").unwrap();

    (1920 <= byr && byr <= 2002)
        && (2010 <= iyr && iyr <= 2020)
        && (2020 <= eyr && eyr <= 2030)
        && valid_hgt(hgt_raw)
        && valid_hair(hcl_raw)
        && valid_eye(eye)
        && valid_passport(pid)
}

fn part_1(data: &Vec<HashMap<String, String>>) -> usize {
    data.iter().filter(|x| valid(x)).count()
}

fn part_2(data: &Vec<HashMap<String, String>>) -> usize {
    data.iter().filter(|x| security_checks(x)).count()
}

fn parse_line(line: String) -> HashMap<String, String> {
    line.trim()
        .split(" ")
        .map(|x| x.split(":"))
        .map(|mut i| (i.next().unwrap().to_string(), i.next().unwrap().to_string()))
        .collect()
}

fn main() {
    let options = Options::from_args();
    let mut reader = BufReader::new(File::open(options.input).unwrap());
    let mut contents = String::new();
    reader.read_to_string(&mut contents);
    let input: Vec<HashMap<String, String>> = contents
        .split("\n\n")
        .map(|x| x.replace("\n", " "))
        .map(parse_line)
        .collect();

    println!("Part 1 = {:?}", part_1(&input));
    println!("Part 2 = {:?}", part_2(&input));
}
