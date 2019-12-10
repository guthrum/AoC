use lib::int_code::{read_file, machine::Machine, machine};
use lib::permutation;

enum Phase {
    Ready,
    ReadSetting,
    ReadInput,
}

struct AmpIo {
    phase_setting: i64,
    input_sgnal: i64,
    phase: Phase,
    outputs: Vec<i64>,
}

impl AmpIo {
    fn new(phase_setting: i64, amp_input: i64) -> Self {
        AmpIo {
            phase_setting,
            input_sgnal: amp_input,
            phase: Phase::Ready,
            outputs: Vec::new(),
        }
    }

    fn get_outputs(&self) -> &Vec<i64> {
        &self.outputs
    }
}

impl machine::StdIo for AmpIo {
    fn read(&mut self) -> i64 {
        match self.phase {
            Phase::Ready => {
                self.phase = Phase::ReadSetting;
                self.phase_setting
            },
            Phase::ReadSetting => {
                self.phase = Phase::ReadInput;
                self.input_sgnal
            },
            _ => panic!("invalid state for reading.")
        }
    }

    fn write(&mut self, value: i64) {
        self.outputs.push(value);
    }
}

struct AmpController {
    program: Vec<i64>,
}

impl AmpController {
    fn new(program: Vec<i64>) -> Self {
        AmpController {
            program,
        }
    }

    fn execute_seq(&self, sequence: &Vec<i64>, intial_input: i64) -> i64 {
        let mut last_output = intial_input;

        for i in 0..5 {
            let phase = sequence.get(i).expect("no phase found");
            let mut amp_io = AmpIo::new(phase.clone(), last_output);
            let mut amp = Machine::new(self.program.clone(), &mut amp_io);
            amp.execute();
            last_output = amp_io.get_outputs().get(0).expect("no machine output").clone();
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
    let input = read_file("/home/tim/projects/AoC19/resources/day7input").expect("failed to read input");
    let amp_controller = AmpController::new(input);
    println!("final output = {:?}", amp_controller.optimise_phases_1());

}