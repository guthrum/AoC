use lib::int_code::{machine::Machine, monitor::Monitor, read_file};
use std::sync::mpsc;
use std::thread;

fn main() {
    let program =
        read_file("/home/tim/projects/AoC19/resources/day13input").expect("failed to read input");
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();
    let mut machine = Machine::new(program, input_rx, output_tx);
    let monitor = Monitor::new(output_rx);
    thread::spawn(move || {
        machine.execute().expect("failed excuting machine");
    });
    monitor.start();
}
