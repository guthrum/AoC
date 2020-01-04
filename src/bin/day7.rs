use lib::int_code::{machine::Machine, read_file};
use lib::permutation;
use std::sync::mpsc;
use std::thread;

struct AmpController {
    program: Vec<i64>,
}

impl AmpController {
    fn new(program: Vec<i64>) -> Self {
        AmpController { program }
    }

    fn execute_seq(&self, sequence: &[i64], intial_input: i64) -> i64 {
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
            machine.execute().expect("failed to execute");
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

    fn execute_feedback_seq(&self, sequence: &[i64], initial_input: i64) -> i64 {
        let (input_machine_1, input_1) = mpsc::channel();
        let (output_1, input_2) = mpsc::channel();
        let (output_2, input_3) = mpsc::channel();
        let (output_3, input_4) = mpsc::channel();
        let (output_4, input_5) = mpsc::channel();
        let (output_5, output_machine_5) = mpsc::channel();

        input_machine_1
            .send(sequence[0])
            .expect("failed to send phase to machine 1");
        input_machine_1
            .send(initial_input)
            .expect("failed to send initial input");
        output_1
            .send(sequence[1])
            .expect("failed to send pahse to machine 2");
        output_2
            .send(sequence[2])
            .expect("failed to send pahse to machine 3");
        output_3
            .send(sequence[3])
            .expect("failed to send pahse to machine 4");
        output_4
            .send(sequence[4])
            .expect("failed to send pahse to machine 5");

        let mut machine_1 = Machine::new(self.program.clone(), input_1, output_1);
        let mut machine_2 = Machine::new(self.program.clone(), input_2, output_2);
        let mut machine_3 = Machine::new(self.program.clone(), input_3, output_3);
        let mut machine_4 = Machine::new(self.program.clone(), input_4, output_4);
        let mut machine_5 = Machine::new(self.program.clone(), input_5, output_5);

        thread::spawn(move || {
            machine_1.execute().expect("failed to execute machine 1");
        });
        thread::spawn(move || {
            machine_2.execute().expect("failed to execute machine 2");
        });
        thread::spawn(move || {
            machine_3.execute().expect("failed to execute machine 3");
        });
        thread::spawn(move || {
            machine_4.execute().expect("failed to execute machine 4");
        });
        thread::spawn(move || {
            machine_5.execute().expect("failed to execute machine 5");
        });

        let mut value = None;

        while let Ok(result) = output_machine_5.recv() {
            match value {
                Some(v) => {
                    if result > v {
                        value = Some(result);
                    }
                }
                None => value = Some(result),
            }
            #[allow(unused_must_use)]
            {
                input_machine_1.send(result);
            }
        }
        value.expect("no value to return")
    }

    fn optimise_phases_2(&self) -> (Vec<i64>, i64) {
        let mut phases = Vec::new();
        let mut score = 0;

        for permutation in permutation::permutations(vec![5, 6, 7, 8, 9]) {
            let result = self.execute_feedback_seq(&permutation, 0);
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
    println!("final output = {:?}", amp_controller.optimise_phases_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = read_file("/home/tim/projects/AoC19/resources/day7input")
            .expect("failed to read input");
        let amp_controller = AmpController::new(input);
        assert_eq!(amp_controller.optimise_phases_1().1, 70_597);
    }

    #[test]
    fn part2() {
        let input = read_file("/home/tim/projects/AoC19/resources/day7input")
            .expect("failed to read input");
        let amp_controller = AmpController::new(input);
        assert_eq!(amp_controller.optimise_phases_2().1, 30_872_528);
    }
}
