use std::convert::{TryFrom};

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

pub trait StdIo {
    fn read(&mut self) -> i64;

    fn write(&mut self, value: i64);
}

#[derive(Debug)]
pub struct Machine<'a, I: StdIo> {
    state: Vec<i64>,
    io: &'a mut I,
    relative_base: i64,
}

impl<'a, I: StdIo> Machine<'a, I> {
    pub fn new(mut input: Vec<i64>, io: &'a mut I) -> Self {
        input.append(&mut vec![0; MEMORY_SIZE - input.len()]);
        Machine {
            state: input,
            io,
            relative_base: 0,
        }
    }

    fn _generate_operation_vec(&self, instruction: i64) -> Option<(i64, usize, usize, usize)> {
        let mut instruc_str: String = instruction.to_string();
        if instruc_str.len() < INSTRUCTION_LENGTH {
            instruc_str = format!("{}{}", String::from_utf8(
                vec![b'0'; INSTRUCTION_LENGTH - instruc_str.len()]).expect("failed to create padding string"), instruc_str);
        }
        let read_mode_3: usize = usize::try_from(instruc_str.remove(0).to_digit(10)?).ok()?;
        let read_mode_2: usize = usize::try_from(instruc_str.remove(0).to_digit(10)?).ok()?;
        let read_mode_1: usize = usize::try_from(instruc_str.remove(0).to_digit(10)?).ok()?;
        let opcode = instruc_str.parse().ok()?;

        Some((opcode, read_mode_1, read_mode_2, read_mode_3))
    }

    fn _create_addressing_mode(mode: usize, value: i64) -> AddressingMode {
        match mode {
            0 => AddressingMode::Register(value as usize),
            1 => AddressingMode::Immediate(value),
            2 => AddressingMode::Relative(value),
            _ => panic!("unrecognised memory mode {}", mode),
        }
    }

    fn _parse_slice(&self, slice: &[i64]) -> Option<(Command, usize)> {
        let op_vec = self._generate_operation_vec(slice[0])?;
        match op_vec.0 {
            1 => Some((Command::Add(
                Self::_create_addressing_mode(op_vec.1, slice[1]),
                Self::_create_addressing_mode(op_vec.2, slice[2]),
                Self::_create_addressing_mode(op_vec.3, slice[3]),
            ), 4)),
            2 => Some((Command::Multiply(
                Self::_create_addressing_mode(op_vec.1, slice[1]),
                Self::_create_addressing_mode(op_vec.2, slice[2]),
                Self::_create_addressing_mode(op_vec.3, slice[3]),
            ), 4)),
            3 => Some((Command::IoRead(
                Self::_create_addressing_mode(op_vec.1, slice[1]),
            ), 2)),
            4 => Some((Command::IoWrite(
                Self::_create_addressing_mode(op_vec.1, slice[1]),
            ), 2)),
            5 => Some((Command::JmpIfTrue(
                Self::_create_addressing_mode(op_vec.1, slice[1]),
                Self::_create_addressing_mode(op_vec.2, slice[2]),
            ), 3)),
            6 => Some((Command::JmpIfFalse(
                Self::_create_addressing_mode(op_vec.1, slice[1]),
                Self::_create_addressing_mode(op_vec.2, slice[2]),
            ), 3)),
            7 => Some((Command::LessThan(
                Self::_create_addressing_mode(op_vec.1, slice[1]),
                Self::_create_addressing_mode(op_vec.2, slice[2]),
                Self::_create_addressing_mode(op_vec.3, slice[3]),
            ), 4)),
            8 => Some((Command::Equal(
                Self::_create_addressing_mode(op_vec.1, slice[1]),
                Self::_create_addressing_mode(op_vec.2, slice[2]),
                Self::_create_addressing_mode(op_vec.3, slice[3]),
            ), 4)),
            9 => {
                println!("op 9: {} : {:?} {:?}", self.relative_base, op_vec, slice);
                Some((Command::AdjustRelativeBase(
                    Self::_create_addressing_mode(op_vec.1, slice[1]),
                ), 2))
            },
            99 => Some((Command::End(), 1)),
            _ => None
        }
    }

    fn _read_memory(&self, addressing_mode: AddressingMode)-> i64 {
        match addressing_mode {
            AddressingMode::Register(pos) => self.state[pos],
            AddressingMode::Immediate(value) => value,
            AddressingMode::Relative(value) => {
                println!("rel {}+{}={}", value, self.relative_base, self.relative_base+value);
                self.state[(self.relative_base + value) as usize]
            }
        }
    }

    fn _write_memory(&mut self, addressing_mode: AddressingMode, value: i64) {
        match addressing_mode {
            AddressingMode::Register(pos) => self.state[pos] = value,
            AddressingMode::Immediate(_) => panic!("can't write value."),
            AddressingMode::Relative(value) => self.state[(self.relative_base + value) as usize] = value,
        }
    }

    fn _read_input(&mut self, addressing_mode: AddressingMode) {
        println!("read: {:?} {}", addressing_mode, self.relative_base);
        let input: i64 = self.io.read();
        self._write_memory(addressing_mode, input);
    }

    fn _write_output(&mut self, addressing_mode: AddressingMode) {
        println!("write: {:?} {}", addressing_mode, self.relative_base);
        self.io.write(self._read_memory(addressing_mode));
    }

    fn _two_arg_test(&self, arg1_mode: AddressingMode, arg2_mode: AddressingMode, test: impl Fn(i64, i64) -> bool) -> bool {
        let arg1 = self._read_memory(arg1_mode);
        let arg2 = self._read_memory(arg2_mode);
        test(arg1, arg2)
    }

    fn _run_machine(&mut self) {
        let mut program_counter = 0;
        let mut parsed_command = self._parse_slice(&self.state[program_counter .. program_counter+4]);
        while let Some(command) = parsed_command {
            match command.0 {
                Command::End() => return,
                Command::Add(v1, v2, res) => {
                    self._write_memory(res, self._read_memory(v1) + self._read_memory(v2));
                    program_counter += command.1;
                },
                Command::Multiply(v1, v2, res) => {
                    self._write_memory(res, self._read_memory(v1) * self._read_memory(v2));
                    program_counter += command.1;
                },
                Command::LessThan(arg1, arg2, res) => {
                    self._write_memory(res, self._two_arg_test(arg1, arg2,|v1, v2| -> bool { v1 < v2 }) as i64);
                    program_counter += command.1;
                },
                Command::Equal(arg1, arg2, res) => {
                    self._write_memory(res, self._two_arg_test(arg1, arg2,|v1, v2| -> bool { v1 == v2 }) as i64);
                    program_counter += command.1;
                },
                Command::IoRead(pos) => {
                    self._read_input(pos);
                    program_counter += command.1;
                },
                Command::IoWrite(pos) => {
                    self._write_output(pos);
                    program_counter += command.1;
                },
                Command::JmpIfTrue(test, ptr) => {
                    if self._read_memory(test) != 0 { 
                        program_counter = self._read_memory(ptr) as usize
                    } else {
                        program_counter += command.1;
                    }
                },
                Command::JmpIfFalse(test, ptr) => {
                    if self._read_memory(test) == 0 { 
                        program_counter = self._read_memory(ptr) as usize
                    } else {
                        program_counter += command.1;
                    }
                },
                Command::AdjustRelativeBase(amount_address) => {
                    let amount = self._read_memory(amount_address);
                    println!("addr: {:?}, v: {}", amount_address, amount);
                    self.relative_base += amount;
                    program_counter += command.1;
                }
            }
            parsed_command = self._parse_slice(&self.state[program_counter .. std::cmp::min(program_counter+4, self.state.len())]);
        }
    }

    pub fn execute(&mut self) {
        self._run_machine();
    }

    pub fn read_memory(&self) -> &Vec<i64> {
        &self.state
    }
}