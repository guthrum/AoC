use std::fmt::Debug;

#[derive(Clone, Copy)]
struct RedTile {
    x: i64,
    y: i64,
}

impl Debug for RedTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl RedTile {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn area(&self, other: &Self) -> u64 {
        let dx = self.x.abs_diff(other.x) + 1;
        let dy = self.y.abs_diff(other.y) + 1;
        dx * dy
    }
}

impl From<&str> for RedTile {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(",").unwrap();
        Self {
            x: i64::from_str_radix(x.trim(), 10).expect("failed to parse x"),
            y: i64::from_str_radix(y.trim(), 10).expect("failed to parse y"),
        }
    }
}

struct Box<'a> {
    p1: &'a RedTile,
    p2: &'a RedTile,
}

impl<'a> Box<'a> {
    fn area(&self) -> u64 {
        self.p1.area(self.p2)
    }
}

struct Line {
    p1: RedTile,
    p2: RedTile,
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}->{:?}", &self.p1, &self.p2))
    }
}

impl Line {
    fn new(p1: RedTile, p2: RedTile) -> Self {
        Self { p1, p2 }
    }

    fn from_xy(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        Self::new(RedTile::new(x1, y1), RedTile::new(x2, y2))
    }

    fn is_point_on_line(&self, p: &RedTile) -> bool {
        if self.p1.x == self.p2.x {
            // check we are same x and between y
            p.x == self.p1.x && self.p1.y.min(self.p2.y) <= p.y && p.y <= self.p1.y.max(self.p2.y)
        } else {
            // check we are the same y and between x
            p.y == self.p1.y && self.p1.x.min(self.p2.x) <= p.x && p.x <= self.p1.x.max(self.p2.x)
        }
    }

    fn intersects(&self, other: &Line) -> bool {
        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
        let x1 = self.p1.x;
        let y1 = self.p1.y;
        let x2 = self.p2.x;
        let y2 = self.p2.y;
        let x3 = other.p1.x;
        let y3 = other.p1.y;
        let x4 = other.p2.x;
        let y4 = other.p2.y;

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denom == 0 {
            false
        } else {
            let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) as f64 / denom as f64;
            let u = (((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) as f64 / denom as f64) * -1.0;
            let res = 0.0 <= t && t <= 1.0 && 0.0 <= u && u <= 1.0;
            println!("{:?} intersects {:?} = {}", &self, other, res);
            res
        }
    }
}

fn get_edges(red_tiles: &[RedTile]) -> Vec<Line> {
    // this does not connect the last with the first
    let mut res: Vec<_> = red_tiles
        .iter()
        .zip(red_tiles.iter().skip(1))
        .map(|(p1, p2)| Line {
            p1: p1.clone(),
            p2: p2.clone(),
        })
        .collect();
    res.push(Line {
        p1: red_tiles.last().unwrap().clone(),
        p2: red_tiles[0].clone(),
    });

    res
}

fn point_inside_test<'a>(edges: &[Line], p: RedTile, bounding_box: &Box<'a>) -> bool {
    let outside_points = vec![
        RedTile::new(p.x, bounding_box.p1.y),
        RedTile::new(p.x, bounding_box.p2.y),
        RedTile::new(bounding_box.p1.x, p.y),
        RedTile::new(bounding_box.p2.x, p.y),
    ];

    for outside_point in outside_points.into_iter() {
        let intersection_count = edges
            .iter()
            .filter(|edge| Line::new(p, outside_point).intersects(&edge))
            .count();
        println!(
            "{:?} -> {:?} intersects {}",
            p, outside_point, intersection_count
        );
        if intersection_count % 2 == 0 {
            return false;
        }
    }
    true
}

fn box_contains_no_external_points<'a>(
    edges: &[Line],
    test: Box<'a>,
    bounding_box: &Box<'a>,
) -> bool {
    // for each point
    // check its inside the edge list
    println!("box test: {:?}->{:?}", test.p1, test.p2);

    for x in test.p1.x.min(test.p2.x)..=test.p2.x.max(test.p1.x) {
        for y in test.p1.y.min(test.p2.y)..=test.p2.y.max(test.p1.y) {
            let p = RedTile::new(x, y);
            let point_on_edge = edges.iter().filter(|e| e.is_point_on_line(&p)).next();
            if let Some(e) = point_on_edge {
                println!("point {:?} on edge {:?} thus inside", &p, e);
                continue;
            }
            println!("testing point {},{}", x, y);
            if !point_inside_test(&edges, p, &bounding_box) {
                println!("Point {},{} not inside", x, y);
                return false;
            }
        }
    }
    true
}

fn read_input(raw: String) -> Vec<RedTile> {
    raw.lines().map(RedTile::from).collect()
}

fn part1(red_tiles: &[RedTile]) -> u64 {
    let mut res = 0;
    for (i, tile1) in red_tiles.iter().enumerate() {
        for j in (i + 1)..red_tiles.len() {
            res = res.max(tile1.area(&red_tiles[j]));
        }
    }
    res
}

fn part2(red_tiles: &[RedTile]) -> u64 {
    // lets get the bounding box
    let mut bounding_box_p1 = red_tiles[0].clone();
    let mut bounding_box_p2 = red_tiles[0].clone();
    for t in red_tiles {
        bounding_box_p1.x = bounding_box_p1.x.min(t.x);
        bounding_box_p1.y = bounding_box_p1.y.min(t.y);
        bounding_box_p2.x = bounding_box_p2.x.min(t.x);
        bounding_box_p2.y = bounding_box_p2.y.min(t.y);
    }
    bounding_box_p1.x -= 1;
    bounding_box_p1.y -= 1;
    bounding_box_p2.x += 1;
    bounding_box_p2.y += 1;

    let edges = get_edges(red_tiles);

    let bounding_box = Box {
        p1: &bounding_box_p1,
        p2: &bounding_box_p2,
    };

    let mut res = 0;
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let bx = Box {
                p1: &red_tiles[i],
                p2: &red_tiles[j],
            };
            let area = bx.area();
            if box_contains_no_external_points(&edges, bx, &bounding_box) {
                println!(
                    "Testing box {:?} -> {:?} size = {}",
                    red_tiles[i], red_tiles[j], area
                );
                res = res.max(area);
            }
        }
    }
    res
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("missing file as first argument");
    let contents = std::fs::read_to_string(file).expect("failed to read file");
    let red_tiles = read_input(contents);
    let p1 = part1(&red_tiles);
    println!("Part 1 = {}", p1);
    println!("Part 1 = {}", part2(&red_tiles));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_line_intersects() {
        let l1 = Line::new(RedTile::new(-10, -10), RedTile::new(10, 10));
        let l2 = Line::new(RedTile::new(10, -10), RedTile::new(-10, 10));
        let l3 = Line::new(RedTile::new(-10, 10), RedTile::new(0, 1));

        assert!(l1.intersects(&l2));
        assert!(l2.intersects(&l1));
        assert!(!l1.intersects(&l3));
        assert!(l2.intersects(&l3));
    }

    #[test]
    fn test_box_contains_no_external_points() {
        let edges = vec![
            Line::from_xy(0, 0, 0, 10),
            Line::from_xy(0, 10, 2, 10),
            Line::from_xy(2, 10, 2, 6),
            Line::from_xy(2, 6, 4, 6),
            Line::from_xy(4, 6, 4, 10),
            Line::from_xy(4, 10, 10, 10),
            Line::from_xy(10, 10, 10, 0),
            Line::from_xy(10, 0, 0, 0),
        ];
        let bb1 = RedTile::new(-1, -1);
        let bb2 = RedTile::new(11, 11);
        let bb = Box { p1: &bb1, p2: &bb2 };

        let test1 = Box {
            p1: &edges[0].p1,
            p2: &edges[2].p1,
        };
        assert!(box_contains_no_external_points(&edges, test1, &bb));

        assert!(!point_inside_test(&edges, RedTile::new(3, 10), &bb));
        let test2 = Box {
            p1: &edges[2].p1,
            p2: &edges[4].p1,
        };
        assert!(
            !box_contains_no_external_points(&edges, test2, &bb),
            "box contains external points"
        );
    }
}
