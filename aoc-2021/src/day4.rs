use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Board {
    locations: HashMap<u8, (u8, u8)>,
    rows: [u8; 5],
    columns: [u8; 5],
    won: bool,
}

impl Board {
    fn new(rows: Vec<Vec<u8>>) -> Self {
        let mut locations = HashMap::with_capacity(25);
        for (y, row) in rows.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                locations.insert(*value, (x as u8, y as u8));
            }
        }
        Board {
            locations,
            rows: [0; 5],
            columns: [0; 5],
            won: false,
        }
    }

    fn mark(&mut self, number: u8) -> Option<u64> {
        match self.locations.remove(&number) {
            Some((x, y)) => {
                self.columns[x as usize] += 1;
                self.rows[y as usize] += 1;
                if self.rows[y as usize] == 5 || self.columns[x as usize] == 5 {
                    let result = self.locations.keys().map(|v| *v as u64).sum();
                    self.won = true;
                    Some(result)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn has_won(&self) -> bool {
        self.won
    }
}

fn parse(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = input.split("\n\n");
    let call_order: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| u8::from_str_radix(s, 10).unwrap())
        .collect();
    let mut boards = Vec::with_capacity(100);
    for board in lines {
        let mut board_values = Vec::with_capacity(5);
        board
            .lines()
            .into_iter()
            .map(|line| {
                let mut row_values = Vec::with_capacity(5);
                line.split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|v| u8::from_str_radix(v, 10).unwrap())
                    .for_each(|v| row_values.push(v));
                row_values
            })
            .for_each(|v| board_values.push(v));
        boards.push(Board::new(board_values));
    }

    (call_order, boards)
}

fn solve(input: &str) -> (u64, u64) {
    let (call_order, mut boards) = parse(input);
    let mut first_win = None;
    let mut last_win = None;
    let mut call_order_iter = call_order.into_iter().peekable();
    while call_order_iter.peek().is_some() {
        let call = call_order_iter.next().unwrap();
        for board in &mut boards {
            if !board.has_won() {
                match board.mark(call) {
                    None => {}
                    Some(bscore) => {
                        if first_win.is_none() {
                            first_win = Some(bscore * (call as u64));
                        }
                        last_win = Some(bscore * (call as u64));
                    }
                }
            }
        }
    }

    (first_win.unwrap(), last_win.unwrap())
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
