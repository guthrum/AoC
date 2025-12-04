// if true has roll
type Row = Vec<bool>;

type Grid = Vec<Row>;

const DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn read_input(contents: String) -> Grid {
    let parse_line = |l: &str| l.chars().map(|c| c == '@').collect();
    contents.lines().map(parse_line).collect()
}

fn part1(grid: &Grid) -> usize {
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;
    let check_pos = |x: i32, y: i32| {
        let mut neighbour_count = 0;
        for (dx, dy) in DELTAS {
            let cx = x + dx;
            let cy = y + dy;
            if cx < 0 || cy < 0 || cx >= max_x || cy >= max_y {
                continue;
            }
            let value = grid[cy as usize][cx as usize];
            if value {
                neighbour_count += 1;
            }
        }
        neighbour_count < 4
    };
    let mut sum = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y as usize][x as usize] && check_pos(x, y) {
                sum += 1;
            }
        }
    }
    sum
}

fn part2(grid: &mut Grid) -> usize {
    let mut grand_total = 0;
    loop {
        let max_y = grid.len() as i32;
        let max_x = grid[0].len() as i32;
        let check_pos = |x: i32, y: i32| {
            let mut neighbour_count = 0;
            for (dx, dy) in DELTAS {
                let cx = x + dx;
                let cy = y + dy;
                if cx < 0 || cy < 0 || cx >= max_x || cy >= max_y {
                    continue;
                }
                let value = grid[cy as usize][cx as usize];
                if value {
                    neighbour_count += 1;
                }
            }
            neighbour_count < 4
        };
        let mut points = Vec::new();
        for y in 0..max_y {
            for x in 0..max_x {
                if grid[y as usize][x as usize] && check_pos(x, y) {
                    points.push((x, y));
                }
            }
        }

        if points.len() == 0 {
            break;
        }
        grand_total += points.len();
        for (x, y) in points {
            grid[y as usize][x as usize] = false
        }
    }
    grand_total
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("missing file as first argument");
    let contents = std::fs::read_to_string(file).expect("failed to read file");
    let mut input = read_input(contents);
    println!("Part 1 = {}", part1(&input));
    println!("Part 2 = {}", part2(&mut input));
}
