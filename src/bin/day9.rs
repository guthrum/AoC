use std::sync::mpsc::{self, Sender, Receiver};
use lib::int_code::{read_file, machine::Machine, machine};

fn solve(program: Vec<i64>) {
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();
    input_tx.send(1).expect("failed to send data");
    let mut machine = Machine::new(program, input_rx, output_tx);
    machine.execute();
    for output in output_rx {
        println!("{}", output);
    }
}


fn main() {
    let input = read_file("/home/tim/projects/AoC19/resources/day9input").expect("failed to read input");
    solve(input);
}