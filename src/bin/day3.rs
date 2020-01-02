use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

#[derive(Clone, Copy, Eq)]
struct Point {
    x: i32,
    y: i32,
    step: i32,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {} in {})", self.x, self.y, self.step)
    }
}

impl Point {
    fn manhattan_distance_to(&self, p: &Point) -> i32 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
enum LineType {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl LineType {
    pub fn next_point(&self, p: &Point, step: i32) -> Point {
        match *self {
            LineType::UP => Point {
                x: p.x,
                y: p.y + 1,
                step,
            },
            LineType::DOWN => Point {
                x: p.x,
                y: p.y - 1,
                step,
            },
            LineType::LEFT => Point {
                x: p.x - 1,
                y: p.y,
                step,
            },
            LineType::RIGHT => Point {
                x: p.x + 1,
                y: p.y,
                step,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    line_type: LineType,
    length: i32,
    current_point: Point,
    step_count: i32,
}

impl Line {
    pub fn new(mut value: String, point: Point) -> Self {
        let line_type = match value.remove(0) {
            'U' => LineType::UP,
            'D' => LineType::DOWN,
            'L' => LineType::LEFT,
            'R' => LineType::RIGHT,
            x => panic!("invalid line type {}", x),
        };
        let length = value.parse().expect("unable to parse line length");
        Line {
            line_type,
            length,
            current_point: point,
            step_count: point.step,
        }
    }
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length < 1 {
            None
        } else {
            self.length -= 1;
            self.step_count += 1;
            self.current_point = self
                .line_type
                .next_point(&self.current_point, self.step_count);
            Some(Point {
                x: self.current_point.x,
                y: self.current_point.y,
                step: self.step_count,
            })
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    path: Vec<String>,
    last_point: Point,
    current_line: Line,
}

impl Path {
    pub fn new(mut path: Vec<String>) -> Self {
        let current_line = Line::new(
            path.remove(0),
            Point {
                x: 0,
                y: 0,
                step: 0,
            },
        );
        Path {
            path,
            last_point: Point {
                x: 0,
                y: 0,
                step: 0,
            },
            current_line,
        }
    }
}

impl Iterator for Path {
    type Item = Point; // output points

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_line.next() {
            Some(point) => {
                self.last_point = point;
                Some(point)
            }
            None => {
                if self.path.is_empty() {
                    return None;
                }
                self.current_line = Line::new(self.path.remove(0), self.last_point);
                self.current_line.next()
            }
        }
    }
}

fn read_file(path: &str) -> std::io::Result<(Vec<String>, Vec<String>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines().map(|l| l.unwrap());
    let path1 = lines_iter
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.to_owned())
        .collect();
    let path2 = lines_iter
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.to_owned())
        .collect();
    Ok((path1, path2))
}

fn get_set(points: Vec<Point>) -> HashSet<Point> {
    let mut set = HashSet::new();
    for point in points {
        if !set.contains(&point) {
            set.insert(point);
        }
    }
    set
}

fn solve(path1: Vec<Point>, path2: Vec<Point>) -> i32 {
    let p1_point_set: HashSet<Point> = get_set(path1);
    let p2_point_set: HashSet<Point> = get_set(path2);
    let origin = Point {
        x: 0,
        y: 0,
        step: 0,
    };

    let reduced_points1: HashMap<(i32, i32), i32> = p1_point_set
        .iter()
        .filter(|x| p2_point_set.contains(x))
        .map(|x| ((x.x, x.y), x.step))
        .collect();

    p2_point_set
        .iter()
        .filter(|x| reduced_points1.contains_key(&(x.x, x.y)))
        .map(|x| reduced_points1.get(&(x.x, x.y)).expect("issue") + x.step)
        .min()
        .unwrap_or(-1)
}

fn main() {
    let paths =
        read_file("/home/tim/projects/AoC19/resources/day3input").expect("failed to load input");
    let path1: Vec<Point> = Path::new(paths.0).into_iter().collect();
    let path2: Vec<Point> = Path::new(paths.1).into_iter().collect();
    println!("{}", solve(path1, path2));
}
