use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashSet, BTreeMap};
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn calculate_hash_of_angle_from_north(from: &Point, to: &Point) -> i64 {
    let dy = (-1*(to.y - from.y)) as f64;
    let dx = (to.x - from.x) as f64;
    let atan = (dx/dy).atan().to_degrees();
    let v = (if (0.0 <= dx) && (0.0 <= dy) {
        atan
    } else if (dx < 0.0) && (0.0 <= dy) {
        360.0 + atan
    } else if (dx < 0.0) && (dy < 0.0) {
        180.0 + atan
    } else {
        180.0 + atan
    } * 1024.0 * 1024.0) as i64;
    v
}

fn calculate_euclidean_distance(from: &Point, to: &Point) -> f64 {
    (((from.x-to.x) as f64).powi(2) + ((from.y-to.y) as f64).powi(2)).sqrt()
}

fn calculate_visible_asteroids(point: &Point, points: &Vec<Point>) -> usize {
    let visible_angles: HashSet<i64> = HashSet::from_iter(points.iter()
        .filter(|p| !(p.x == point.x && p.y == point.y))
        .map(|x| calculate_hash_of_angle_from_north(&point, x)));
    visible_angles.len()
}

fn find_optimal_point(points: &Vec<Point>) -> (&Point, usize) {
    let mut point = &points[0];
    let mut count = 0;
    for p in points {
        let num = calculate_visible_asteroids(&p, points);
        if num > count {
            count = num;
            point = p;
        }
    }
    (point, count)
}

fn read_input(path: &str) -> Option<Vec<Point>> {
    let f = File::open(path).ok()?;
    let reader = BufReader::new(f);
    let mut points = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        for (x, cell) in line.ok()?.chars().enumerate() {
            match cell {
                '.' => {},
                '#' => {points.push(Point{ x: x as i32, y: y as i32})},
                _ => panic!("unknown input type"),
            }
        }
    }
    Some(points)
}


fn insert_in_sorted_vec(vec: &mut Vec<(f64, Point)>, p: (f64, Point)) {
    for i in 0..vec.len() {
        if p.0 > vec[i].0 {
            vec.insert(i, p); 
            return;
        }
    }
    vec.push(p);
}

struct Station {
    asteroids: Vec<Point>,
    position: Point,
}

impl Station {
    fn position_at_optimal_place(mut points: Vec<Point>) -> Self {
        let best_station = find_optimal_point(&points).0.clone();
        let pos = points.iter().position(|p| p.x == best_station.x && p.y == best_station.y).unwrap();
        points.remove(pos);
        Station {
            asteroids: points,
            position: best_station,
        }
    }

    fn solve(&self) -> Vec<Point> {
        // calculate map
        let mut angle_to_list_of_asteroids: BTreeMap<i64, Vec<(f64, Point)>> = BTreeMap::new();
        for p in &self.asteroids {
            let angle = calculate_hash_of_angle_from_north(&self.position, p);
            let distance_from_station = calculate_euclidean_distance(&self.position, p);
            let vec = angle_to_list_of_asteroids.entry(angle).or_insert_with(Vec::new);
            // println!("{:?} {} {}", p, angle, distance_from_station);
            insert_in_sorted_vec(vec, (distance_from_station, p.clone()));
        }

        let mut destroyed_order = Vec::new();
        let mut complete = false;
        while !complete {
            complete = true;
            for (_, value) in angle_to_list_of_asteroids.iter_mut() {
                if value.len() > 0 {
                    let next = value.pop().unwrap().1;
                    destroyed_order.push(next);
                    complete = false;
                }
            }
        }
        destroyed_order
    }
}


fn main() {
    let input = read_input("/home/tim/projects/AoC19/resources/day10input").expect("failed to load input");
    println!("{:?}", find_optimal_point(&input));
    let station = Station::position_at_optimal_place(input);
    let destroyed = station.solve();
    println!("{:?}", destroyed[199]);
    println!("{}", destroyed[199].x*100 + destroyed[199].y);
}
