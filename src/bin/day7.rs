use lib::int_code::{machine::Machine, read_file};
use lib::permutation;
use std::sync::mpsc;

struct AmpController {
    program: Vec<i64>,
}

impl AmpController {
    fn new(program: Vec<i64>) -> Self {
        AmpController { program }
    }

    fn execute_seq(&self, sequence: &Vec<i64>, intial_input: i64) -> i64 {
        let mut last_output = intial_input;

        for i in 0..5 {
            let phase = sequence.get(i).expect("no phase found");
            let (input_tx, input_rx) = mpsc::channel();
            let (output_tx, output_rx) = mpsc::channel();
            input_tx.send(*phase).expect("failed to send data");
            input_tx
                .send(last_output.clone())
                .expect("failed to send data");
            let mut machine = Machine::new(self.program.clone(), input_rx, output_tx);
            machine.execute();
            last_output = output_rx.recv().expect("failed to receive");
        }
        last_output
    }

    fn optimise_phases_1(&self) -> (Vec<i64>, i64) {
        let mut phases = Vec::new();
        let mut score = 0;

        for permutation in permutation::permutations(vec![0, 1, 2, 3, 4]) {
            let result = self.execute_seq(&permutation, 0);
            if result > score {
                score = result;
                phases = permutation;
            }
        }

        (phases, score)
    }

    fn optimise_phases_2(&self) -> (Vec<i64>, i64) {
        let mut phases = Vec::new();
        let mut score = 0;

        for permutation in permutation::permutations(vec![5, 6, 7, 8, 9]) {
            let result = self.execute_seq(&permutation, 0);
            if result > score {
                score = result;
                phases = permutation;
            }
        }

        (phases, score)
    }
}

fn main() {
    let input =
        read_file("/home/tim/projects/AoC19/resources/day7input").expect("failed to read input");
    let amp_controller = AmpController::new(input);
    println!("final output = {:?}", amp_controller.optimise_phases_1());
}
