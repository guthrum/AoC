use lib::int_code::{read_file, machine::Machine, machine};
use std::io;

struct SimpleIo {
}

impl machine::StdIo for SimpleIo {
    fn read(&mut self) -> i64 {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("failed to read line");
        let buffer = buffer.trim();
        let input: i64 = buffer.parse().expect(&format!("failed to parse {} to i32.", buffer));
        input
    }

    fn write(&mut self, value: i64) {
        println!("output = {}", value);
    }
}

fn solve(program: Vec<i64>) {
    let mut io = SimpleIo{};
    let mut machine = Machine::new(program, &mut io);
    machine.execute();
}


fn main() {
    let input = read_file("/home/tim/projects/AoC19/resources/day9input").expect("failed to read input");
    solve(input);
}