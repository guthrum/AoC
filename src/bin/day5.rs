use std::convert::TryFrom;
use std::fs;
use std::io::{self};

static INSTRUCTION_LENGTH: usize = 5;

#[derive(Copy, Clone, Debug)]
enum AddressingMode {
    Register(usize),
    Immediate(i32),
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
    IoRead(usize),
    IoWrite(usize),
}

fn read_file(path: &str) -> std::io::Result<Vec<i32>> {
    Ok(fs::read_to_string(path)?
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect())
}

#[derive(Debug)]
struct Machine {
    state: Vec<i32>,
}

impl Machine {
    fn new(input: Vec<i32>) -> Self {
        Machine { state: input }
    }

    fn _generate_operation_vec(instruction: i32) -> Option<(i32, usize, usize, usize)> {
        let mut instruc_str: String = instruction.to_string();
        if instruc_str.len() < INSTRUCTION_LENGTH {
            instruc_str = format!(
                "{}{}",
                String::from_utf8(vec![b'0'; INSTRUCTION_LENGTH - instruc_str.len()])
                    .expect("failed to create padding string"),
                instruc_str
            );
        }
        let read_mode_3: usize = usize::try_from(instruc_str.remove(0).to_digit(10)?).ok()?;
        let read_mode_2: usize = usize::try_from(instruc_str.remove(0).to_digit(10)?).ok()?;
        let read_mode_1: usize = usize::try_from(instruc_str.remove(0).to_digit(10)?).ok()?;
        let opcode: i32 = instruc_str.parse().ok()?;

        Some((opcode, read_mode_1, read_mode_2, read_mode_3))
    }

    fn _create_addressing_mode(mode: usize, value: i32) -> AddressingMode {
        match mode {
            1 => AddressingMode::Immediate(value),
            0 => AddressingMode::Register(value as usize),
            _ => panic!("unrecognised memory mode {}", mode),
        }
    }

    fn _parse_slice(slice: &[i32]) -> Option<(Command, usize)> {
        let op_vec = Self::_generate_operation_vec(slice[0])?;
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
            3 => Some((Command::IoRead(slice[1] as usize), 2)),
            4 => Some((Command::IoWrite(slice[1] as usize), 2)),
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
            99 => Some((Command::End(), 1)),
            _ => None,
        }
    }

    fn _read_memory(&self, addressing_mode: AddressingMode) -> i32 {
        match addressing_mode {
            AddressingMode::Immediate(value) => value,
            AddressingMode::Register(pos) => self.state[pos],
        }
    }

    fn _write_memory(&mut self, addressing_mode: AddressingMode, value: i32) {
        match addressing_mode {
            AddressingMode::Immediate(_) => panic!("can't write value."),
            AddressingMode::Register(pos) => self.state[pos] = value,
        }
    }

    fn _read_input(&mut self, pos: usize) {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read line");
        let buffer = buffer.trim();
        let input: i32 = buffer
            .parse()
            .unwrap_or_else(|_| panic!("failed to parse {} to i32.", buffer));
        self.state[pos] = input;
    }

    fn _write_output(&self, pos: usize) {
        println!("output {}={:?}", pos, self.state.get(pos));
    }

    fn _two_arg_test(
        &self,
        arg1_mode: AddressingMode,
        arg2_mode: AddressingMode,
        test: impl Fn(i32, i32) -> bool,
    ) -> bool {
        let arg1 = self._read_memory(arg1_mode);
        let arg2 = self._read_memory(arg2_mode);
        test(arg1, arg2)
    }

    fn run_machine(&mut self) -> i32 {
        let mut program_counter = 0;
        let mut parsed_command =
            Self::_parse_slice(&self.state[program_counter..program_counter + 4]);
        while let Some(command) = parsed_command {
            match command.0 {
                Command::End() => return self.state[0],
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
                        self._two_arg_test(arg1, arg2, |v1, v2| -> bool { v1 < v2 }) as i32,
                    );
                    program_counter += command.1;
                }
                Command::Equal(arg1, arg2, res) => {
                    self._write_memory(
                        res,
                        self._two_arg_test(arg1, arg2, |v1, v2| -> bool { v1 == v2 }) as i32,
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
            }
            parsed_command = Self::_parse_slice(
                &self.state[program_counter..std::cmp::min(program_counter + 4, self.state.len())],
            );
        }
        0
    }
}

fn main() {
    let mut machine = Machine::new(
        read_file("/home/tim/projects/AoC19/resources/day5input").expect("unable to load numbers"),
    );
    machine.run_machine();
}
