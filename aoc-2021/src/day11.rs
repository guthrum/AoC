use std::fs::read_to_string;

const FLASH_THRESHOLD: i16 = 9;
const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

struct Octopus {
    grid: [i16; GRID_WIDTH * GRID_HEIGHT],
}

impl Octopus {
    fn perform_iteration(&mut self) -> usize {
        let mut affected: Vec<(i32, i32)> = Vec::new();
        // increment by 1 and track which are going to flash
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let pos = (x + GRID_WIDTH * y) as usize;
                self.grid[pos] += 1;
                if self.grid[pos] > FLASH_THRESHOLD {
                    affected.push((x as i32, y as i32));
                }
            }
        }
        // trigger flashes
        while let Some((x, y)) = affected.pop() {
            let pos = (x + GRID_WIDTH as i32 * y) as usize;
            // since a cell could be triggered multiple times we skip if already handled.
            if self.grid[pos] < 0 {
                continue;
            }

            self.grid[pos] = -1;
            // check all the neighbours
            for (nx, ny) in vec![
                (x + 1, y),
                (x - 1, y),
                (x, y + 1),
                (x, y - 1),
                (x - 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y - 1),
                (x + 1, y + 1),
            ] {
                // if neighbour is invalid skip
                if nx < 0 || nx > GRID_WIDTH as i32 - 1 || ny < 0 || ny > GRID_HEIGHT as i32 - 1 {
                    continue;
                }
                let npos = (nx + GRID_WIDTH as i32 * ny) as usize;
                // if the neighbour has already flashed the value will be below 0
                if !self.grid[npos] < 0 {
                    self.grid[npos] += 1;
                    if self.grid[npos] > FLASH_THRESHOLD {
                        affected.push((nx as i32, ny as i32));
                    }
                }
            }
        }
        // now we need to go though and reset each value that has flashed.
        // and tally the total up
        let mut count = 0;
        for v in &mut self.grid {
            if *v < 0 {
                count += 1;
                *v = 0;
            }
        }
        count
    }
}

fn read_input(path: &str) -> Octopus {
    let initial_states: Vec<i16> = read_to_string(path)
        .unwrap()
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| i16::from_str_radix(&c.to_string(), 10).unwrap())
        })
        .collect();
    let grid: [i16; GRID_WIDTH * GRID_HEIGHT] = initial_states.try_into().unwrap();
    Octopus { grid }
}

fn solve(mut input: Octopus) -> (usize, u64) {
    let mut p1_count = 0;
    let mut all_flash = 0;
    let mut step = 1;
    while all_flash == 0 || step <= 100 {
        let tally = input.perform_iteration();
        if tally == 100 && all_flash == 0 {
            all_flash = step;
        }
        if step <= 100 {
            p1_count += tally;
        }
        step += 1;
    }

    (p1_count, all_flash)
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let input = read_input(&file_path);
    let (p1, p2) = solve(input);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
