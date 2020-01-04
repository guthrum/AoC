use std::convert::TryFrom;
use std::sync::mpsc::{Receiver, Sender};

static INSTRUCTION_LENGTH: usize = 5;
static MEMORY_SIZE: usize = 4096;

#[derive(Copy, Clone, Debug)]
enum AddressingMode {
    Register(usize),
    Immediate(i64),
    Relative(i64),
}

#[derive(Copy, Clone, Debug)]
enum Command {
    End(),
    Add(AddressingMode, AddressingMode, AddressingMode),
    Multiply(AddressingMode, AddressingMode, AddressingMode),
    JmpIfTrue(AddressingMode, AddressingMode),
    JmpIfFalse(AddressingMode, AddressingMode),
    LessThan(AddressingMode, AddressingMode, AddressingMode),
    Equal(AddressingMode, AddressingMode, AddressingMode),
    IoRead(AddressingMode),
    IoWrite(AddressingMode),
    AdjustRelativeBase(AddressingMode),
}

type MachineMemoryType = i64;

#[derive(Debug, Clone)]
pub struct MachineError {
    reason: String,
}

impl std::fmt::Display for MachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "machine failed error msg: {}", self.reason)
    }
}

impl std::error::Error for MachineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct Machine {
    state: Vec<MachineMemoryType>,
    relative_base: i64,
    input: Receiver<MachineMemoryType>,
    output: Sender<MachineMemoryType>,
}

impl Machine {
    pub fn new(
        mut program: Vec<MachineMemoryType>,
        input: Receiver<MachineMemoryType>,
        output: Sender<MachineMemoryType>,
    ) -> Self {
        program.append(&mut vec![0; MEMORY_SIZE - program.len()]);
        Machine {
            state: program,
            relative_base: 0,
            input,
            output,
        }
    }

    fn _generate_operation_vec(
        &self,
        instruction: MachineMemoryType,
    ) -> Option<(MachineMemoryType, usize, usize, usize)> {
        let opcode = instruction % 100;
        let read_mode_1 = (instruction / 100) % 10;
        let read_mode_2 = (instruction / 1000) % 10;
        let read_mode_3 = (instruction / 10000) % 10;
        Some((
            opcode,
            read_mode_1 as usize,
            read_mode_2 as usize,
            read_mode_3 as usize,
        ))
    }

    fn _create_addressing_mode(mode: usize, value: MachineMemoryType) -> AddressingMode {
        match mode {
            0 => AddressingMode::Register(value as usize),
            1 => AddressingMode::Immediate(value),
            2 => AddressingMode::Relative(value),
            _ => panic!("unrecognised memory mode {}", mode),
        }
    }

    fn _parse_slice(&self, slice: &[MachineMemoryType]) -> Option<(Command, usize)> {
        let op_vec = self._generate_operation_vec(slice[0])?;
        match op_vec.0 {
            1 => Some((
                Command::Add(
                    Self::_create_addressing_mode(op_vec.1, slice[1]),
                    Self::_create_addressing_mode(op_vec.2, slice[2]),
                    Self::_create_addressing_mode(op_vec.3, slice[3]),
                ),
                4,
            )),
            2 => Some((
                Command::Multiply(
                    Self::_create_addressing_mode(op_vec.1, slice[1]),
                    Self::_create_addressing_mode(op_vec.2, slice[2]),
                    Self::_create_addressing_mode(op_vec.3, slice[3]),
                ),
                4,
            )),
            3 => Some((
                Command::IoRead(Self::_create_addressing_mode(op_vec.1, slice[1])),
                2,
            )),
            4 => Some((
                Command::IoWrite(Self::_create_addressing_mode(op_vec.1, slice[1])),
                2,
            )),
            5 => Some((
                Command::JmpIfTrue(
                    Self::_create_addressing_mode(op_vec.1, slice[1]),
                    Self::_create_addressing_mode(op_vec.2, slice[2]),
                ),
                3,
            )),
            6 => Some((
                Command::JmpIfFalse(
                    Self::_create_addressing_mode(op_vec.1, slice[1]),
                    Self::_create_addressing_mode(op_vec.2, slice[2]),
                ),
                3,
            )),
            7 => Some((
                Command::LessThan(
                    Self::_create_addressing_mode(op_vec.1, slice[1]),
                    Self::_create_addressing_mode(op_vec.2, slice[2]),
                    Self::_create_addressing_mode(op_vec.3, slice[3]),
                ),
                4,
            )),
            8 => Some((
                Command::Equal(
                    Self::_create_addressing_mode(op_vec.1, slice[1]),
                    Self::_create_addressing_mode(op_vec.2, slice[2]),
                    Self::_create_addressing_mode(op_vec.3, slice[3]),
                ),
                4,
            )),
            9 => Some((
                Command::AdjustRelativeBase(Self::_create_addressing_mode(op_vec.1, slice[1])),
                2,
            )),
            99 => Some((Command::End(), 1)),
            _ => None,
        }
    }

    fn _read_memory(&self, addressing_mode: AddressingMode) -> MachineMemoryType {
        match addressing_mode {
            AddressingMode::Register(pos) => self.state[pos],
            AddressingMode::Immediate(value) => value,
            AddressingMode::Relative(offset) => self.state[(self.relative_base + offset) as usize],
        }
    }

    fn _write_memory(&mut self, addressing_mode: AddressingMode, value: MachineMemoryType) {
        match addressing_mode {
            AddressingMode::Register(pos) => self.state[pos] = value,
            AddressingMode::Immediate(_) => panic!("can't write value."),
            AddressingMode::Relative(offset) => {
                self.state[(self.relative_base + offset) as usize] = value
            }
        }
    }

    fn _read_input(&mut self, addressing_mode: AddressingMode) {
        // println!("read: {:?} {}", addressing_mode, self.relative_base);
        match self.input.recv() {
            Ok(input) => self._write_memory(addressing_mode, input),
            Err(_) => panic!("input closed before machine finished"),
        }
    }

    fn _write_output(&mut self, addressing_mode: AddressingMode) {
        // println!("write: {:?} {}", addressing_mode, self.relative_base);
        match self.output.send(self._read_memory(addressing_mode)) {
            Ok(()) => {}
            Err(e) => panic!(format!("failed to send data: {}", e)),
        }
    }

    fn _two_arg_test(
        &self,
        arg1_mode: AddressingMode,
        arg2_mode: AddressingMode,
        test: impl Fn(MachineMemoryType, MachineMemoryType) -> bool,
    ) -> bool {
        let arg1 = self._read_memory(arg1_mode);
        let arg2 = self._read_memory(arg2_mode);
        test(arg1, arg2)
    }

    pub fn execute(&mut self) -> Result<(), MachineError> {
        let mut program_counter = 0;
        let mut parsed_command =
            self._parse_slice(&self.state[program_counter..program_counter + 4]);
        while let Some(command) = parsed_command {
            match command.0 {
                Command::End() => {
                    return Ok(());
                }
                Command::Add(v1, v2, res) => {
                    self._write_memory(res, self._read_memory(v1) + self._read_memory(v2));
                    program_counter += command.1;
                }
                Command::Multiply(v1, v2, res) => {
                    self._write_memory(res, self._read_memory(v1) * self._read_memory(v2));
                    program_counter += command.1;
                }
                Command::LessThan(arg1, arg2, res) => {
                    self._write_memory(
                        res,
                        self._two_arg_test(arg1, arg2, |v1, v2| -> bool { v1 < v2 }) as i64,
                    );
                    program_counter += command.1;
                }
                Command::Equal(arg1, arg2, res) => {
                    self._write_memory(
                        res,
                        self._two_arg_test(arg1, arg2, |v1, v2| -> bool { v1 == v2 }) as i64,
                    );
                    program_counter += command.1;
                }
                Command::IoRead(pos) => {
                    self._read_input(pos);
                    program_counter += command.1;
                }
                Command::IoWrite(pos) => {
                    self._write_output(pos);
                    program_counter += command.1;
                }
                Command::JmpIfTrue(test, ptr) => {
                    if self._read_memory(test) != 0 {
                        program_counter = self._read_memory(ptr) as usize
                    } else {
                        program_counter += command.1;
                    }
                }
                Command::JmpIfFalse(test, ptr) => {
                    if self._read_memory(test) == 0 {
                        program_counter = self._read_memory(ptr) as usize
                    } else {
                        program_counter += command.1;
                    }
                }
                Command::AdjustRelativeBase(amount_address) => {
                    let amount = self._read_memory(amount_address);
                    self.relative_base += amount;
                    program_counter += command.1;
                }
            }
            parsed_command = self._parse_slice(
                &self.state[program_counter..std::cmp::min(program_counter + 4, self.state.len())],
            );
        }
        Err(MachineError {
            reason: String::from("ran out of instructions."),
        })
    }

    pub fn read_memory(&self) -> &Vec<MachineMemoryType> {
        &self.state
    }

    pub fn read_relative_base(&self) -> MachineMemoryType {
        self.relative_base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{self};
    use std::sync::mpsc;

    fn read_file(path: &str) -> io::Result<Vec<i64>> {
        Ok(fs::read_to_string(path)?
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect())
    }

    #[test]
    fn day2_example_1() {
        let (_, input_rx) = mpsc::channel();
        let (output_tx, _) = mpsc::channel();
        let mut machine = Machine::new(vec![1, 0, 0, 0, 99], input_rx, output_tx);
        machine.execute();
        assert_eq!(machine.read_memory()[0], 2);
    }

    #[test]
    fn day2_part1() {
        let (_, input_rx) = mpsc::channel();
        let (output_tx, _) = mpsc::channel();
        let mut machine = Machine::new(
            vec![
                1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 6, 1, 19, 1, 5, 19, 23, 2, 9,
                23, 27, 1, 6, 27, 31, 1, 31, 9, 35, 2, 35, 10, 39, 1, 5, 39, 43, 2, 43, 9, 47, 1,
                5, 47, 51, 1, 51, 5, 55, 1, 55, 9, 59, 2, 59, 13, 63, 1, 63, 9, 67, 1, 9, 67, 71,
                2, 71, 10, 75, 1, 75, 6, 79, 2, 10, 79, 83, 1, 5, 83, 87, 2, 87, 10, 91, 1, 91, 5,
                95, 1, 6, 95, 99, 2, 99, 13, 103, 1, 103, 6, 107, 1, 107, 5, 111, 2, 6, 111, 115,
                1, 115, 13, 119, 1, 119, 2, 123, 1, 5, 123, 0, 99, 2, 0, 14, 0,
            ],
            input_rx,
            output_tx,
        );
        machine.execute();
        assert_eq!(machine.read_memory()[0], 3101844);
    }

    #[test]
    fn day5_example1() {
        let (_, input_rx) = mpsc::channel();
        let (output_tx, _) = mpsc::channel();
        let mut machine = Machine::new(vec![1002, 4, 3, 4, 33], input_rx, output_tx);
        machine.execute();
        assert_eq!(machine.read_memory()[4], 99);
    }

    #[test]
    fn day5_example_1() {
        for (input, output) in vec![(1, 1), (0, 0)] {
            let (input_tx, input_rx) = mpsc::channel();
            let (output_tx, output_rx) = mpsc::channel();
            input_tx.send(input).expect("failed to send data");
            let mut machine = Machine::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                input_rx,
                output_tx,
            );
            machine.execute();
            assert_eq!(output_rx.recv().expect("failed to read output"), output);
        }
    }

    #[test]
    fn day5_example_2() {
        for (input, output) in vec![(7, 999), (8, 1000), (9, 1001)] {
            let (input_tx, input_rx) = mpsc::channel();
            let (output_tx, output_rx) = mpsc::channel();
            input_tx.send(input).expect("failed to send data");
            let mut machine = Machine::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                input_rx,
                output_tx,
            );
            machine.execute();
            assert_eq!(output_rx.recv().expect("failed to read output"), output);
        }
    }

    #[test]
    fn day5_part2() {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        input_tx.send(5).expect("failed to send data");
        let program = read_file("/home/tim/projects/AoC19/resources/day5input")
            .expect("failed to read day 5 in");
        let mut machine = Machine::new(program, input_rx, output_tx);
        machine.execute();
        assert_eq!(output_rx.recv().expect("failed to read output"), 773660);
    }

    #[test]
    fn day9_example1() {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        input_tx.send(5).expect("failed to send data");
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut machine = Machine::new(program.clone(), input_rx, output_tx);
        machine.execute();
        let output: Vec<MachineMemoryType> = output_rx.try_iter().collect();
        assert_eq!(program, output);
    }

    #[test]
    fn day9_example2() {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        input_tx.send(5).expect("failed to send data");
        let mut machine = Machine::new(
            vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0],
            input_rx,
            output_tx,
        );
        machine.execute();
        assert_eq!(
            format!("{}", output_rx.recv().expect("failed to read output")).len(),
            16
        );
    }

    #[test]
    fn day9_example3() {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        input_tx.send(5).expect("failed to send data");
        let mut machine = Machine::new(vec![104, 1125899906842624, 99], input_rx, output_tx);
        machine.execute();
        assert_eq!(
            output_rx.recv().expect("failed to read output"),
            1125899906842624
        );
    }

    #[test]
    fn day9_part1() {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        input_tx.send(1).expect("failed to send data");
        let program = read_file("/home/tim/projects/AoC19/resources/day9input")
            .expect("failed to read day 9 in");
        let mut machine = Machine::new(program.clone(), input_rx, output_tx);
        machine.execute();
        assert_eq!(output_rx.try_recv().expect("expect output"), 3906448201);
    }

    #[test]
    fn day9_part2() {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        input_tx.send(2).expect("failed to send data");
        let program = read_file("/home/tim/projects/AoC19/resources/day9input")
            .expect("failed to read day 9 in");
        let mut machine = Machine::new(program.clone(), input_rx, output_tx);
        machine.execute();
        assert_eq!(output_rx.try_recv().expect("expect output"), 59785);
    }
}
