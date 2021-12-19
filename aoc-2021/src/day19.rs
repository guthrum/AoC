use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type Point = (i32, i32, i32);
type PointDelta = (i32, i32, i32);
type ScanInfo = HashMap<u32, Vec<Point>>;

const ROTATION_MATRICES: [[[i32; 3]; 3]; 24] = [
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
];

fn rotate_point(p: Point, matrix: &[[i32; 3]; 3]) -> Point {
    (
        matrix[0][0] * p.0 + matrix[0][1] * p.1 + matrix[0][2] * p.2,
        matrix[1][0] * p.0 + matrix[1][1] * p.1 + matrix[1][2] * p.2,
        matrix[2][0] * p.0 + matrix[2][1] * p.1 + matrix[2][2] * p.2,
    )
}

fn sub_points(p0: Point, p1: Point) -> Point {
    (p0.0 - p1.0, p0.1 - p1.1, p0.2 - p1.2)
}

fn add_points(p0: Point, p1: Point) -> Point {
    (p0.0 + p1.0, p0.1 + p1.1, p0.2 + p1.2)
}

fn read_input(path: &str) -> ScanInfo {
    let raw = read_to_string(path).unwrap();
    let mut lines = raw.lines().peekable();

    let mut scan_info = HashMap::new();

    while lines.peek().is_some() {
        let scanner_id = u32::from_str_radix(
            &lines
                .next()
                .unwrap()
                .replace("--- scanner ", "")
                .replace(" ---", ""),
            10,
        )
        .unwrap();
        while let Some(line) = lines.next() {
            if line == "" {
                break;
            }
            let parts: Vec<i32> = line
                .split(",")
                .map(|p| i32::from_str_radix(p, 10).unwrap())
                .collect();
            scan_info
                .entry(scanner_id)
                .or_insert(Vec::new())
                .push((parts[0], parts[1], parts[2]));
        }
    }

    scan_info
}

fn get_delta(points: &[Point]) -> HashMap<PointDelta, Vec<(Point, Point)>> {
    let mut res = HashMap::with_capacity(points.len());

    for (idx, p) in points.iter().enumerate() {
        for p2 in points.iter().skip(idx + 1) {
            let delta = (p.0 - p2.0, p.1 - p2.1, p.2 - p2.2);
            res.entry(delta).or_insert(Vec::new()).push((*p, *p2));
        }
    }

    res
}

fn solve(input: ScanInfo) -> (usize, i32) {
    let mut p0_points: Vec<(i32, i32, i32)> = input.get(&0).unwrap().iter().cloned().collect();
    let mut p0_deltas = get_delta(&p0_points);
    let mut scanner_locations: HashMap<u32, Point> = HashMap::new();
    scanner_locations.insert(0, (0, 0, 0));

    let mut to_handle: Vec<u32> = input.keys().cloned().filter(|k| *k != 0).collect();

    while !to_handle.is_empty() {
        let mut still_unsolved = Vec::with_capacity(to_handle.len());

        for scanner_id in to_handle {
            if scanner_id == 0 {
                continue;
            }
            let scanner_points = input.get(&scanner_id).unwrap();
            // TODO: we could only calc this once per scanner point, good idea
            let scanner_deltas = get_delta(&scanner_points);
            // track which deltas match for a given rotation?
            let mut tracker: HashMap<[[i32; 3]; 3], Vec<((Point, Point), (Point, Point))>> =
                HashMap::new();
            for (delta, pair) in scanner_deltas.iter() {
                // now we need to multiply by each rotation matrix to find options
                for rotation in &ROTATION_MATRICES {
                    let rotated = rotate_point(*delta, rotation);
                    if let Some(p0_pair) = p0_deltas.get(&rotated) {
                        for p0_p in p0_pair {
                            for p in pair {
                                tracker
                                    .entry(*rotation)
                                    .or_insert(Vec::with_capacity(200))
                                    .push((*p, *p0_p));
                            }
                        }
                    }
                }
            }
            let opt_rotation = tracker
                .into_iter()
                .filter(|x| x.1.len() >= 12)
                .max_by(|x, y| x.1.cmp(&y.1));
            if opt_rotation.is_none() {
                still_unsolved.push(scanner_id);
                continue;
            }
            let rotation = opt_rotation.unwrap();
            // now to find the location of scanner i, for this we need to find two pairs that have
            // a point in common for both side.
            let mut point_mapping = HashMap::with_capacity(rotation.1.len());
            for pair in rotation.1 {
                point_mapping
                    .entry(pair.1 .0)
                    .or_insert(Vec::new())
                    .push(pair.0);
                point_mapping
                    .entry(pair.1 .1)
                    .or_insert(Vec::new())
                    .push(pair.0);
            }
            let decision_points = point_mapping
                .into_iter()
                .filter(|(_, v)| v.len() >= 2)
                .next()
                .unwrap();
            let dp_1 = decision_points.1.get(0).unwrap();
            let dp_2 = decision_points.1.get(1).unwrap();
            let p0_decision_point = decision_points.0;
            let scanner_decision_point = if dp_1.1 == dp_2.1 || dp_1.1 == dp_2.0 {
                dp_1.1
            } else if dp_1.0 == dp_2.1 || dp_1.0 == dp_2.0 {
                dp_1.0
            } else {
                panic!("failed.")
            };
            let adjusted_scanner_point = rotate_point(scanner_decision_point, &rotation.0);
            let scanner_location = sub_points(p0_decision_point, adjusted_scanner_point);
            scanner_locations.insert(scanner_id, scanner_location);

            // now we need to add point to p0 set and re-calculate the deltas
            let mut new_p0_points: HashSet<Point> = p0_points.iter().cloned().collect();
            for p in scanner_points {
                new_p0_points.insert(add_points(rotate_point(*p, &rotation.0), scanner_location));
            }
            p0_points = new_p0_points.into_iter().collect();
            // TODO: we could do an incremental calc only, instead of complete re-computation
            p0_deltas = get_delta(&p0_points);
        }

        to_handle = still_unsolved;
    }
    let max_distance: Option<i32> = scanner_locations
        .iter()
        .flat_map(|(_, p1)| {
            scanner_locations
                .iter()
                .map(|(_, p2)| (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs())
        })
        .max();

    (p0_points.len(), max_distance.unwrap())
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let input = read_input(&file_path);
    let (p1, p2) = solve(input);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
