use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn next_clockwise(&self) -> Self {
        match *self {
            Heading::North => Heading::East,
            Heading::East => Heading::South,
            Heading::South => Heading::West,
            Heading::West => Heading::North,
        }
    }

    fn next_counter_clockwise(&self) -> Self {
        match *self {
            Heading::North => Heading::West,
            Heading::East => Heading::North,
            Heading::South => Heading::East,
            Heading::West => Heading::South,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Direction {
    North(i64),
    East(i64),
    South(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl Direction {
    fn new_bearing(&self, current: Heading) -> Heading {
        match *self {
            Direction::Right(angle) => match angle {
                0 => current,
                90 => current.next_clockwise(),
                180 => current.next_clockwise().next_clockwise(),
                270 => current.next_counter_clockwise(),
                _ => panic!("Angle {} not expected", angle),
            },
            Direction::Left(angle) => match angle {
                0 => current,
                90 => current.next_counter_clockwise(),
                180 => current.next_clockwise().next_clockwise(),
                270 => current.next_clockwise(),
                _ => panic!("Angle {} not expected", angle),
            },
            _ => current,
        }
    }

    fn rotate_point(&self, x: i64, y: i64) -> (i64, i64) {
        let angle = match *self {
            Direction::Right(angle) => (360 - angle),
            Direction::Left(angle) => angle,
            _ => 0,
        };
        match angle {
            0 => (x, y),
            90 => (-y, x),
            180 => (-x, -y),
            270 => (y, -x),
            _ => panic!("unknown angle {}", angle),
        }
    }
}

impl From<String> for Direction {
    fn from(raw: String) -> Self {
        let action = raw.get(0..=0).unwrap();
        let distance: i64 = raw.get(1..).unwrap().parse().unwrap();
        match action {
            "N" => Direction::North(distance),
            "E" => Direction::East(distance),
            "S" => Direction::South(distance),
            "W" => Direction::West(distance),
            "L" => Direction::Left(distance % 360),
            "R" => Direction::Right(distance % 360),
            "F" => Direction::Forward(distance),
            _ => panic!("Cannot parse {}", raw),
        }
    }
}

#[derive(Debug)]
struct Boat1 {
    heading: Heading,
    x: i64,
    y: i64,
}

impl Boat1 {
    fn new() -> Self {
        Boat1 {
            heading: Heading::East,
            x: 0,
            y: 0,
        }
    }

    fn current_position(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    fn instruction(&mut self, direction: &Direction) {
        match direction {
            Direction::North(dy) => self.y += dy,
            Direction::East(dx) => self.x += dx,
            Direction::South(dy) => self.y -= dy,
            Direction::West(dx) => self.x -= dx,
            Direction::Forward(d) => match self.heading {
                Heading::North => self.y += d,
                Heading::East => self.x += d,
                Heading::South => self.y -= d,
                Heading::West => self.x -= d,
            },
            _ => {
                self.heading = direction.new_bearing(self.heading);
            }
        }
    }
}

fn part_1(directions: &Vec<Direction>) -> i64 {
    let mut boat = Boat1::new();
    for direction in directions {
        boat.instruction(direction);
    }
    let (dx, dy) = boat.current_position();
    dx.abs() + dy.abs()
}

#[derive(Debug)]
struct Boat2 {
    x: i64,
    y: i64,
    wx: i64,
    wy: i64,
}

impl Boat2 {
    fn new() -> Self {
        Boat2 {
            x: 0,
            y: 0,
            wx: 10,
            wy: 1,
        }
    }

    fn current_position(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    fn instruction(&mut self, direction: &Direction) {
        match direction {
            Direction::North(dy) => self.wy += dy,
            Direction::East(dx) => self.wx += dx,
            Direction::South(dy) => self.wy -= dy,
            Direction::West(dx) => self.wx -= dx,
            Direction::Forward(d) => {
                self.x += d * self.wx;
                self.y += d * self.wy;
            }
            _ => {
                let (nx, ny) = direction.rotate_point(self.wx, self.wy);
                self.wx = nx;
                self.wy = ny;
            }
        }
    }
}

fn part_2(directions: &Vec<Direction>) -> i64 {
    let mut boat = Boat2::new();
    for direction in directions {
        boat.instruction(direction);
    }
    let (dx, dy) = boat.current_position();
    dx.abs() + dy.abs()
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let raw: Vec<Direction> = reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| Direction::from(x))
        .collect();
    println!("Part 1 = {}", part_1(&raw));
    println!("Part 2 = {}", part_2(&raw));
}
