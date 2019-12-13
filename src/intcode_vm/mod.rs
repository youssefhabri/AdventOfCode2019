use std::collections::VecDeque;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VMState {
    Halted,
    NeedsInput,
    Output(i64),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Opcode {
    Add(ParamMode, ParamMode, ParamMode),
    Mul(ParamMode, ParamMode, ParamMode),
    Input(ParamMode),
    Output(ParamMode),
    JmpIfZero(ParamMode, ParamMode),
    JmpIfNotZero(ParamMode, ParamMode),
    LessThen(ParamMode, ParamMode, ParamMode),
    Equals(ParamMode, ParamMode, ParamMode),
    SetRB(ParamMode),
    Halt,
    Invalid,
}

impl From<i64> for Opcode {
    fn from(value: i64) -> Self {
        let (opcode, param_mode_1, param_mode_2, param_mode_3) = parse_instruction(value);
        match opcode {
            1 => Opcode::Add(param_mode_1, param_mode_2, param_mode_3),
            2 => Opcode::Mul(param_mode_1, param_mode_2, param_mode_3),
            3 => Opcode::Input(param_mode_1),
            4 => Opcode::Output(param_mode_1),
            5 => Opcode::JmpIfNotZero(param_mode_1, param_mode_2),
            6 => Opcode::JmpIfZero(param_mode_1, param_mode_2),
            7 => Opcode::LessThen(param_mode_1, param_mode_2, param_mode_3),
            8 => Opcode::Equals(param_mode_1, param_mode_2, param_mode_3),
            9 => Opcode::SetRB(param_mode_1),
            99 => Opcode::Halt,
            _ => Opcode::Invalid,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for ParamMode {
    fn from(mode: i64) -> Self {
        match mode {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Invalid ParamMode"),
        }
    }
}

fn parse_instruction(instruction: i64) -> (i64, ParamMode, ParamMode, ParamMode) {
    let opcode = instruction % 100;
    let param_mode_1 = (instruction / 100) % 10;
    let param_mode_2 = (instruction / 1000) % 10;
    let param_mode_3 = (instruction / 10000) % 10;

    (
        opcode,
        ParamMode::from(param_mode_1),
        ParamMode::from(param_mode_2),
        ParamMode::from(param_mode_3),
    )
}

#[derive(Clone, Default, Debug)]
pub struct IntcodeVM {
    memory: Vec<i64>,
    instruction_pointer: usize,
    relative_pointer: usize,
    current_opcode: Option<Opcode>,
    current_params: [i64; 3],
    inputs: VecDeque<i64>,
    jumped: bool,
}

impl IntcodeVM {
    pub fn new() -> Self {
        IntcodeVM {
            memory: vec![],
            instruction_pointer: 0,
            relative_pointer: 0,
            current_opcode: None,
            current_params: [0; 3],
            inputs: VecDeque::new(),
            jumped: false,
        }
    }

    pub fn set_program(&mut self, program: Vec<i64>) {
        self.memory = program;
    }

    pub fn reset(&mut self) {
        self.memory.clear();
        self.instruction_pointer = 0;
        self.relative_pointer = 0;
        self.current_opcode = None;
        self.current_params = [0; 3];
        self.inputs.clear();
        self.jumped = false;
    }

    pub fn push_input(&mut self, value: i64) {
        self.inputs.push_back(value);
    }

    fn load_memory(&self, position: usize) -> i64 {
        match self.memory.get(position) {
            Some(value) => *value,
            None => 0,
        }
    }

    fn load_memory_by_pointer(&self, position: usize) -> i64 {
        self.load_memory(self.load_memory(position) as usize)
    }

    fn save_memory(&mut self, position: usize, new_value: i64) {
        match self.memory.get_mut(position) {
            Some(value) => *value = new_value,
            None => {
                self.memory.resize(position + 1, 0i64);
                self.memory[position] = new_value;
            }
        }
    }

    fn save_memory_by_pointer(&mut self, position: usize, value: i64) {
        self.save_memory(self.load_memory(position) as usize, value)
    }

    fn get_param(&self, mode: ParamMode, offset: usize) -> i64 {
        match mode {
            ParamMode::Immediate => self.load_memory(self.instruction_pointer + offset),
            ParamMode::Position => self.load_memory_by_pointer(self.instruction_pointer + offset),
            ParamMode::Relative => {
                let relative_offset = self.load_memory(self.instruction_pointer + offset) as usize;
                self.load_memory(self.relative_pointer + relative_offset)
            }
        }
    }

    fn set_param(&mut self, mode: ParamMode, offset: usize, value: i64) {
        match mode {
            ParamMode::Immediate => panic!("Immediate mode is not allowed when setting memory!"),
            ParamMode::Position => {
                self.save_memory_by_pointer(self.instruction_pointer + offset, value)
            }
            ParamMode::Relative => {
                let relative_offset = self.load_memory(self.instruction_pointer + offset) as usize;
                self.save_memory(self.relative_pointer + relative_offset, value)
            }
        }
    }

    fn parse_current_opcode(&self) -> Opcode {
        Opcode::from(self.memory[self.instruction_pointer])
    }

    fn advance_parser(&mut self) {
        if !self.jumped {
            self.instruction_pointer += match self.current_opcode.unwrap() {
                Opcode::Add(..) | Opcode::Mul(..) | Opcode::LessThen(..) | Opcode::Equals(..) => 4,
                Opcode::Input(..) | Opcode::Output(..) | Opcode::SetRB(..) => 2,
                Opcode::JmpIfZero(..) | Opcode::JmpIfNotZero(..) => 3,
                Opcode::Halt => 1,
                Opcode::Invalid => panic!("Invalid Opcode!"),
            };
        }

        self.jumped = false;

        self.current_opcode = Some(self.parse_current_opcode());
    }

    pub fn execute(&mut self) -> VMState {
        if self.current_opcode == Some(Opcode::Halt) {
            return VMState::Halted;
        }

        self.current_opcode = Some(self.parse_current_opcode());

        let mut output = None;

        loop {
            if let Some(output) = output {
                return VMState::Output(output);
            }

            match self.current_opcode.unwrap() {
                Opcode::Add(in1, in2, out) => {
                    let result = self.get_param(in1, 1) + self.get_param(in2, 2);
                    self.set_param(out, 3, result);
                }
                Opcode::Mul(in1, in2, out) => {
                    let result = self.get_param(in1, 1) * self.get_param(in2, 2);
                    self.set_param(out, 3, result);
                }
                Opcode::Input(out) => {
                    let value = match self.inputs.pop_front() {
                        Some(x) => x,
                        None => return VMState::NeedsInput,
                    };
                    self.set_param(out, 1, value);
                }
                Opcode::Output(in1) => {
                    output = Some(self.get_param(in1, 1));
                }
                Opcode::JmpIfZero(in1, in2) => {
                    let value = self.get_param(in1, 1);
                    let new_pointer = self.get_param(in2, 2);
                    if value == 0 {
                        self.instruction_pointer = new_pointer as usize;
                        self.jumped = true;
                    }
                }
                Opcode::JmpIfNotZero(in1, in2) => {
                    let value = self.get_param(in1, 1);
                    let new_pointer = self.get_param(in2, 2);
                    if value != 0 {
                        self.instruction_pointer = new_pointer as usize;
                        self.jumped = true;
                    }
                }
                Opcode::LessThen(in1, in2, out) => {
                    let value1 = self.get_param(in1, 1);
                    let value2 = self.get_param(in2, 2);
                    let result = if value1 < value2 { 1 } else { 0 };
                    self.set_param(out, 3, result);
                }
                Opcode::Equals(in1, in2, out) => {
                    let value1 = self.get_param(in1, 1);
                    let value2 = self.get_param(in2, 2);
                    let result = if value1 == value2 { 1 } else { 0 };
                    self.set_param(out, 3, result);
                }
                Opcode::SetRB(in1) => {
                    let offset = self.get_param(in1, 1);
                    self.relative_pointer = (self.relative_pointer as i64 + offset).max(0) as usize;

                    if self.relative_pointer > self.memory.len() {
                        self.memory.resize(self.relative_pointer, 0);
                    }
                }
                Opcode::Halt => return VMState::Halted,
                Opcode::Invalid => panic!("Invalid Opcode!"),
            }

            self.advance_parser();
        }
    }

    pub fn run_once(&mut self) -> Vec<i64> {
        let mut output = vec![];

        loop {
            match self.execute() {
                VMState::Halted => break,
                VMState::NeedsInput => panic!("Not enough input"),
                VMState::Output(value) => output.push(value),
            }
        }

        output
    }
}
