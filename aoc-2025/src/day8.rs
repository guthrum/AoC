use std::collections::HashMap;

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn l2_distance(&self, other: &Point) -> f64 {
        let s =
            (other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2);
        s.sqrt()
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut coords = value.splitn(3, ",");
        let x = u32::from_str_radix(coords.next().unwrap(), 10).expect("failed to parse x") as f64;
        let y = u32::from_str_radix(coords.next().unwrap(), 10).expect("failed to parse y") as f64;
        let z = u32::from_str_radix(coords.next().unwrap(), 10).expect("failed to parse z") as f64;
        Self { x, y, z }
    }
}

fn read_input(contents: String) -> HashMap<usize, Point> {
    contents
        .lines()
        .enumerate()
        .map(|(idx, l)| (idx, Point::from(l)))
        .collect()
}

fn part_1(points: &HashMap<usize, Point>) -> (usize, i64) {
    let mut edges = Vec::with_capacity(points.len().pow(2));
    // build the edge set
    let max_point_id = *points.keys().max().unwrap();
    for (p1_idx, p1_pos) in points {
        for p2_idx in (p1_idx + 1)..=max_point_id {
            let p2_pos = points.get(&p2_idx).unwrap();
            edges.push((p1_idx, p2_idx, p1_pos.l2_distance(p2_pos)));
        }
    }
    edges.sort_by(|f, s| f.2.total_cmp(&s.2));

    let mut cluster_id: u32 = 0;
    let mut node_id_to_cluster: HashMap<usize, u32> = HashMap::with_capacity(1000);
    let mut cluster_id_to_nodes: HashMap<u32, Vec<usize>> = HashMap::new();

    let mut part2_answer = 0;
    let mut part1_answer = 0;

    for (idx, (p1, p2, _)) in edges.iter().enumerate() {
        let connected = match (
            node_id_to_cluster.get(p1).cloned(),
            node_id_to_cluster.get(p2).cloned(),
        ) {
            (Some(cid1), Some(cid2)) => {
                if cid1 != cid2 {
                    // merge cid2 into cid1
                    let nodes = cluster_id_to_nodes.remove(&cid2).unwrap_or_default();
                    let cid1_set = cluster_id_to_nodes.entry(cid1).or_default();
                    for nid in nodes {
                        node_id_to_cluster.insert(nid, cid1);
                        cid1_set.push(nid);
                    }
                    true
                } else {
                    false
                }
            }
            (Some(cid1), None) => {
                node_id_to_cluster.insert(*p2, cid1);
                cluster_id_to_nodes.entry(cid1).and_modify(|v| v.push(*p2));
                true
            }
            (None, Some(cid2)) => {
                node_id_to_cluster.insert(**p1, cid2);
                cluster_id_to_nodes.entry(cid2).and_modify(|v| v.push(**p1));
                true
            }
            (None, None) => {
                node_id_to_cluster.insert(**p1, cluster_id);
                node_id_to_cluster.insert(*p2, cluster_id);
                let v = cluster_id_to_nodes.entry(cluster_id).or_default();
                v.push(**p1);
                v.push(*p2);
                cluster_id += 1;
                true
            }
        };
        if connected {
            part2_answer = (points.get(p1).unwrap().x as i64) * (points.get(p2).unwrap().x as i64);
        }
        if idx == 1000 {
            let mut cluster_sizes: Vec<_> = cluster_id_to_nodes.values().map(|v| v.len()).collect();
            cluster_sizes.sort();
            part1_answer = cluster_sizes.into_iter().rev().take(3).product();
        }
    }

    (part1_answer, part2_answer)
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("missing file as first argument");
    let contents = std::fs::read_to_string(file).expect("failed to read file");
    let points = read_input(contents);
    let (p1, p2) = part_1(&points);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
