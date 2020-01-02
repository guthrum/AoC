use lib::int_code::{machine, machine::Machine, read_file};
use std::sync::mpsc::{self, Receiver, Sender};

fn solve(program: Vec<i64>, input: i64) -> Vec<i64> {
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();
    input_tx.send(input).expect("failed to send data");
    let mut machine = Machine::new(program, input_rx, output_tx);
    machine.execute();
    output_rx.try_iter().collect()
}

fn main() {
    let input =
        read_file("/home/tim/projects/AoC19/resources/day9input").expect("failed to read input");
    println!("part1 outputs = {:?}", solve(input.clone(), 1));
    println!("part2 outputs = {:?}", solve(input.clone(), 2));
}
