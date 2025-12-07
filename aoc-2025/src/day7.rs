use std::collections::HashMap;

#[derive(Clone, PartialEq, Copy)]
enum Cell {
    Source,
    Light(usize),
    Splitter,
    Empty,
}

impl Cell {
    fn is_light(&self) -> bool {
        match *self {
            Self::Light(_) | Self::Source => true,
            _ => false,
        }
    }

    fn count(&self) -> usize {
        match *self {
            Self::Light(c) => c,
            Self::Source => 1,
            _ => 0,
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Source,
            '^' => Self::Splitter,
            _ => Self::Empty,
        }
    }
}

fn read_input(s: String) -> Vec<Vec<Cell>> {
    s.lines()
        .map(|l| l.chars().map(Cell::from).collect())
        .collect()
}

fn soln(mut grid: Vec<Vec<Cell>>) -> (usize, usize) {
    let mut split_count = 0;
    for row_idx in 1..grid.len() {
        let light_indexes: HashMap<usize, usize> = grid[row_idx - 1]
            .iter()
            .enumerate()
            .filter(|v| v.1.is_light())
            .map(|v| (v.0, v.1.count()))
            .collect();
        for cell_idx in 0..grid[row_idx].len() {
            let cell = grid[row_idx][cell_idx];
            if let Some(source) = light_indexes.get(&cell_idx) {
                if cell == Cell::Splitter {
                    grid[row_idx][cell_idx - 1] =
                        Cell::Light(source + grid[row_idx][cell_idx - 1].count());
                    grid[row_idx][cell_idx + 1] =
                        Cell::Light(source + grid[row_idx][cell_idx + 1].count());
                    split_count += 1;
                } else {
                    grid[row_idx][cell_idx] = Cell::Light(source + grid[row_idx][cell_idx].count());
                }
            }
        }
    }
    (
        split_count,
        grid.last().unwrap().iter().map(|v| v.count()).sum(),
    )
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("missing file as first argument");
    let contents = std::fs::read_to_string(file).expect("failed to read file");
    let grid = read_input(contents);
    let (p1, p2) = soln(grid);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
