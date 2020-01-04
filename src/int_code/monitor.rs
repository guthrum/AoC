use std::convert::TryFrom;
use std::sync::mpsc::Receiver;
use std::io::{stdout, Write};
use termion::{color, cursor};
use termion::raw::IntoRawMode;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Copy, Clone, Debug)]
enum DisplayObject {
    Empty,
    Wall,
    Block,
    HorizPaddle,
    Ball,
}

impl TryFrom<i64> for DisplayObject {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DisplayObject::Empty),
            1 => Ok(DisplayObject::Wall),
            2 => Ok(DisplayObject::Block),
            3 => Ok(DisplayObject::HorizPaddle),
            4 => Ok(DisplayObject::Ball),
            _ => Err(format!("no object for {}", value)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Display(Point, DisplayObject),
    Score(i64),
}

impl Instruction {
    fn new(f: i64, s: i64, arg: i64) -> Option<Self> {
        match (f, s, DisplayObject::try_from(arg)) {
            (-1, 0, _) => Some(Instruction::Score(arg)),
            // we add one as termion is 1,1 based not from 0,0 as the monitor is.
            (x, y, Ok(obj)) => Some(Instruction::Display(Point { x: (x+1) as u16, y: (y+1) as u16 }, obj)),
            _ => None,
        }
    }
}

pub struct Monitor {
    input: Receiver<i64>,
}

impl Monitor {
    pub fn new(input: Receiver<i64>) -> Self {
        Monitor { input }
    }

    pub fn start(&self) {
        let mut stdout = stdout().into_raw_mode().expect("failed to get raw terminal");
        write!(stdout, "{}{}{}", termion::clear::All, cursor::Goto(1, 1), termion::cursor::Hide).unwrap();
        loop {
            let instruction = match (self.input.recv(), self.input.recv(), self.input.recv()) {
                (Ok(x), Ok(y), Ok(z)) => match Instruction::new(x, y, z) {
                    Some(ins) => ins,
                    None => panic!("failed to parse {},{},{} into instruction", x, y, z),
                },
                (Ok(i), Ok(j), Err(e)) => panic!("partial instruction {},{} e = {}", i, j, e),
                (Ok(i), Err(e), _) => panic!("partial instruction {} e = {}", i, e),
                _ => break,
            };
            match instruction {
                Instruction::Display(position, obj) => {
                    let pos = cursor::Goto(position.x, position.y);
                    match obj {
                        DisplayObject::Empty => write!(stdout, "{}{} ", color::Fg(color::Reset), pos),
                        DisplayObject::Ball => write!(stdout, "{}{}●", color::Fg(color::Blue), pos),
                        DisplayObject::Wall => write!(stdout, "{}{}█", color::Fg(color::Rgb(211, 211, 211)), pos),
                        DisplayObject::Block => write!(stdout, "{}{}█", color::Fg(color::Red), pos),
                        DisplayObject::HorizPaddle => write!(stdout, "{}{}━", color::Fg(color::Yellow), pos),
                    };
                },
                _ => {}
            }
        }

        write!(stdout, "{}", termion::cursor::Show).unwrap();
    }
}
