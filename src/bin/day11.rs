use std::sync::mpsc::{self, Sender, Receiver};
use lib::int_code::{read_file, machine::Machine, machine};
use std::thread;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Colour {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn change_x(&self) -> i32 {
        match *self {
            Self::East => 1,
            Self::West => -1,
            _ => 0,
        }
    }

    fn change_y(&self) -> i32 {
        match *self {
            Self::North=> 1,
            Self::South => -1,
            _ => 0,
        }
    }

    fn turn(&self, direction: Direction) -> Heading {
        match (*self, direction) {
            (Self::North, Direction::Left) => Heading::West,
            (Self::North, Direction::Right) => Heading::East,
            (Self::East, Direction::Left) => Heading::North,
            (Self::East, Direction::Right) => Heading::South,
            (Self::South, Direction::Left) => Heading::East,
            (Self::South, Direction::Right) => Heading::West,
            (Self::West, Direction::Left) => Heading::South,
            (Self::West, Direction::Right) => Heading::North,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}


#[derive(Debug)]
struct Canvas {
    canvas: HashMap<Coordinate, Colour>
}

impl Canvas {
    fn new() -> Self {
        Canvas {
            canvas: HashMap::new(),
        }
    }

    pub fn paint(&mut self, pos: Coordinate, colour: Colour) {
        self.canvas.insert(pos, colour);
    }

    pub fn get_colour(&self, pos: &Coordinate) -> Option<&Colour> {
        self.canvas.get(pos)
    }

    pub fn print(&self) {
        for y in (-10..10).rev() {
            for x in -40..40 {
                let chr = match self.canvas.get(&Coordinate{ x, y}).unwrap_or(&Colour::Black) {
                    Colour::Black => " ",
                    Colour::White => "X",
                };
                print!("{}", chr);
            }
            println!("");
        }
    }
}

fn solve(program: Vec<i64>, starting_colour: Colour) -> Canvas {
    let mut canvas = Canvas::new();
    let mut position = Coordinate {x: 0, y: 0};
    let mut heading = Heading::North;
    canvas.paint(position.clone(), starting_colour);

    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();
    let mut machine = Machine::new(program, input_rx, output_tx);

    thread::spawn(move || {
        machine.execute();
    });

    loop {
        let input = match canvas.get_colour(&position).unwrap_or(&Colour::Black) {
            Colour::Black => 0,
            Colour::White => 1,
        };
        if input_tx.send(input).is_err() {
            break;
        }
        let colour_to_paint = match output_rx.recv() {
            Ok(0) => Colour::Black,
            Ok(1) => Colour::White,
            Ok(_) => panic!("unknown response for colour to paint"),
            Err(e) => {
                eprintln!("failed to get colour {}", e);
                break
            }
        };
        canvas.paint(position.clone(), colour_to_paint);
        let direction = match output_rx.recv() {
            Ok(0) => Direction::Left,
            Ok(1) => Direction::Right,
            Ok(_) => panic!("unknown response for direction"),
            Err(e) => {
                eprintln!("failed to get direction {}", e);
                break
            }
        };
        heading = heading.turn(direction);
        position.x += heading.change_x();
        position.y += heading.change_y();
    }
    canvas
}

fn main() {
    let input = read_file("/home/tim/projects/AoC19/resources/day11input").expect("failed to read input");
    println!("part1 outputs = {:?}", solve(input.clone(), Colour::Black).canvas.len());
    solve(input.clone(), Colour::White).print();
}
