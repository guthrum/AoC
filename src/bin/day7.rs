use lib::int_code::{read_file, machine::Machine, machine};

enum Phase {
    Ready,
    ReadSetting,
    ReadInput,
}

struct AmpIo {
    phase_setting: i32,
    input_sgnal: i32,
    phase: Phase,
    outputs: Vec<i32>,
}

impl AmpIo {
    fn new(phase_setting: i32, amp_input: i32) -> Self {
        AmpIo {
            phase_setting,
            input_sgnal: amp_input,
            phase: Phase::Ready,
            outputs: Vec::new(),
        }
    }

    fn get_outputs(&self) -> &Vec<i32> {
        &self.outputs
    }
}

impl machine::StdIo for AmpIo {
    fn read(&mut self) -> i32 {
        match self.phase {
            Phase::Ready => {
                self.phase = Phase::ReadSetting;
                self.phase_setting
            },
            Phase::ReadSetting => {
                self.phase = Phase::ReadInput;
                self.input_sgnal
            },
            _ => panic!("unexpected reading in state")
        }
    }

    fn write(&mut self, value: i32) {
        self.outputs.push(value);
    }
}

struct AmpController {
    program: Vec<i32>,
}

impl AmpController {
    fn new(program: Vec<i32>) -> Self {
        AmpController {
            program,
        }
    }

    fn execute_seq(&self, sequence: Vec<i32>, intial_input: i32) -> i32 {
        let mut last_output = intial_input;

        for i in 0..5 {
            let phase = sequence.get(i).expect("no phase found");
            println!("running amp {} in -> {}, phase -> {}", i, last_output, phase);
            let mut amp_io = AmpIo::new(phase.clone(), last_output);
            let mut amp = Machine::new(self.program.clone(), &mut amp_io);
            amp.execute();
            last_output = amp_io.get_outputs().get(0).expect("no machine output").clone();
        }

        last_output
    }

}
fn main() {
    let input = read_file("/home/tim/projects/AoC19/resources/day7input").expect("failed to read input");
    let amp_controller = AmpController::new(input);
    let seq = vec![0,1,2,3,4];
    println!("final output = {}", amp_controller.execute_seq(seq, 0));
}