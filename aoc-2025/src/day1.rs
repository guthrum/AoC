use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left(v) => f.write_fmt(format_args!("L{}", v)),
            Self::Right(v) => f.write_fmt(format_args!("R{}", v)),
        }
    }
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.chars().next().expect("require first char is direction");
        let amount =
            i32::from_str_radix(&s[1..], 10).expect("require rest of the string to be a number");
        match direction {
            'R' => Ok(Rotation::Right(amount)),
            'L' => Ok(Rotation::Left(amount)),
            d => Err(format!("direction {d} is invalid")),
        }
    }
}

#[derive(Debug)]
struct Dial {
    value: i32,
}

impl Display for Dial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

impl Dial {
    fn rotate(&self, rotation: &Rotation) -> Self {
        let mut value = match rotation {
            Rotation::Left(v) => {
                let adjustment = v.min(&self.value);
                let new_value = self.value - adjustment;
                if new_value == 0 {
                    100 - ((v - adjustment) % 100)
                } else {
                    new_value
                }
            }
            Rotation::Right(v) => (self.value + v) % 100,
        };
        value = value % 100;

        Self { value }
    }
}

fn part1(input: &[Rotation]) -> usize {
    let mut dial = Dial { value: 50 };
    let mut zero_count = 0;
    for rotation in input {
        dial = dial.rotate(&rotation);
        if dial.value == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("missing file as first argument");
    let contents = std::fs::read_to_string(file).expect("failed to read file");
    let rotations_or_err: Result<Vec<_>, _> =
        contents.lines().map(|l| Rotation::from_str(l)).collect();
    let rotations = rotations_or_err.expect("failed to parse input");

    let p1 = part1(&rotations);

    println!("Part 1: {}", p1);
}
