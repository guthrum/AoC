#[derive(Debug)]
struct RedTile {
    x: u64,
    y: u64,
}

impl RedTile {
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
            x: u64::from_str_radix(x.trim(), 10).expect("failed to parse x"),
            y: u64::from_str_radix(y.trim(), 10).expect("failed to parse y"),
        }
    }
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

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("missing file as first argument");
    let contents = std::fs::read_to_string(file).expect("failed to read file");
    let red_tiles = read_input(contents);
    let p1 = part1(&red_tiles);
    println!("Part 1 = {}", p1);
}
