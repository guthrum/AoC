use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match *self {
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
            Direction::North => (0, -1),
            Direction::South => (0, 1),
        }
    }

    fn invert(&self) -> Self {
        match *self {
            Direction::West => Self::East,
            Direction::East => Self::West,
            Direction::North => Self::South,
            Direction::South => Self::North,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
enum TileKind {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    Start,
}

impl From<char> for TileKind {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::BendNE,
            'J' => Self::BendNW,
            '7' => Self::BendSW,
            'F' => Self::BendSE,
            '.' => Self::Ground,
            'S' => Self::Start,
            c => panic!("cannot parse {c}"),
        }
    }
}

impl TileKind {
    fn connected_directions(&self) -> Vec<Direction> {
        match *self {
            Self::Start => vec![
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::West,
            ],
            Self::Horizontal => vec![Direction::East, Direction::West],
            Self::Vertical => vec![Direction::North, Direction::South],
            Self::BendSE => vec![Direction::East, Direction::South],
            Self::BendNW => vec![Direction::North, Direction::West],
            Self::BendSW => vec![Direction::South, Direction::West],
            Self::BendNE => vec![Direction::East, Direction::North],
            Self::Ground => vec![],
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Tile {
    kind: TileKind,
    x: usize,
    y: usize,
}

impl Tile {
    fn connected_neighbours<'a>(&self, input: &'a Vec<Vec<Tile>>) -> Vec<&'a Tile> {
        let my_directions = self.kind.connected_directions();
        my_directions
            .into_iter()
            .map(|direction| (direction.clone(), direction.offset()))
            .map(|(direction, (dx, dy))| {
                let x = ((self.x as i32) + dx) as usize;
                let y = ((self.y as i32) + dy) as usize;
                (direction, input.get(y).unwrap().get(x).unwrap())
            })
            .filter(|(direction, neighbour)| neighbour.is_connected(direction.invert()))
            .map(|(_, neighbour)| neighbour)
            .collect()
    }

    fn is_connected(&self, direction: Direction) -> bool {
        self.kind.connected_directions().contains(&direction)
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| Tile {
                    kind: TileKind::from(c),
                    x,
                    y,
                })
                .collect()
        })
        .collect()
}

fn find_loop(input: &Vec<Vec<Tile>>) -> Vec<Tile> {
    // find the start....
    let start = input
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| c.kind == TileKind::Start)
                .map(|(_, t)| t)
                .next()
        })
        .next()
        .unwrap();
    let mut loop_tiles = Vec::new();
    let mut visited = HashSet::new();

    let mut current = start;
    while loop_tiles.is_empty() || current.kind != TileKind::Start {
        loop_tiles.push(current.clone());
        visited.insert(current.clone());
        current = current
            .connected_neighbours(&input)
            .into_iter()
            .filter(|t| !visited.contains(t))
            .next()
            .unwrap_or(start);
    }

    loop_tiles
}

fn scanline_count_is_even(
    width: usize,
    height: usize,
    tile: &Tile,
    tile_loop: &HashSet<&Tile>,
    tiles: &Vec<Vec<Tile>>,
    dx: i32,
    dy: i32,
) -> bool {
    let mut nx = tile.x as i32;
    let mut ny = tile.y as i32;
    nx += dx;
    ny += dy;

    let mut tile_loop_count = 0;

    // while within bounds
    while nx < (width as i32) && nx >= 0 && ny < (height as i32) && ny >= 0 {
        let tt = tiles.get(ny as usize).unwrap().get(nx as usize).unwrap();
        if tile_loop.contains(tt) {
            tile_loop_count += 1;
        }
        nx += dx;
        ny += dy;
    }

    tile_loop_count % 2 == 0
}

fn find_internal_tiles<'a>(tile_loop_array: &[Tile], tiles: &'a Vec<Vec<Tile>>) -> Vec<&'a Tile> {
    let tile_loop: HashSet<&Tile> = HashSet::from_iter(tile_loop_array.iter());
    let height = tiles.len();
    let width = tiles.get(0).unwrap().len();

    let mut internal_tiles = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let tile = tiles.get(y).unwrap().get(x).unwrap();
            if !tile_loop.contains(tile) {
                let x_line = scanline_count_is_even(width, height, tile, &tile_loop, &tiles, -1, 0)
                    && scanline_count_is_even(width, height, tile, &tile_loop, &tiles, 1, 0);
                let y_line = scanline_count_is_even(width, height, tile, &tile_loop, &tiles, 0, -1)
                    && scanline_count_is_even(width, height, tile, &tile_loop, &tiles, 0, 1);
                if !x_line && !y_line {
                    internal_tiles.push(tile);
                }
            }
        }
    }

    internal_tiles
}

fn solve(input: &str) -> (usize, usize) {
    let tiles = parse_input(&input);

    let loop_tiles = find_loop(&tiles);
    // doesn't work
    let internal_tiles = find_internal_tiles(&loop_tiles, &tiles);
    let p1 = (loop_tiles.len() + 1) / 2;
    let p2 = internal_tiles.len();
    (p1, p2)
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_input() {
        let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        assert_eq!(solve(input), (0, 0));
    }
}
