use std::collections::VecDeque;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VMState {
    Halted,
    AwaitsInput,
    Output(i32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Opcode {
    Add(ParamMode, ParamMode, ParamMode),
    Mul(ParamMode, ParamMode, ParamMode),
    Input(ParamMode),
    Output(ParamMode),
    JmpEZ(ParamMode, ParamMode),
    JmpNEZ(ParamMode, ParamMode),
    LessThen(ParamMode, ParamMode, ParamMode),
    Equals(ParamMode, ParamMode, ParamMode),
    Halt,
    Invalid,
}

impl From<i32> for Opcode {
    fn from(value: i32) -> Self {
        let (opcode, param_mode_1, param_mode_2, param_mode_3) = parse_instruction(value);
        match opcode {
            1 => Opcode::Add(param_mode_1, param_mode_2, param_mode_3),
            2 => Opcode::Mul(param_mode_1, param_mode_2, param_mode_3),
            3 => Opcode::Input(param_mode_1),
            4 => Opcode::Output(param_mode_1),
            5 => Opcode::JmpNEZ(param_mode_1, param_mode_2),
            6 => Opcode::JmpEZ(param_mode_1, param_mode_2),
            7 => Opcode::LessThen(param_mode_1, param_mode_2, param_mode_3),
            8 => Opcode::Equals(param_mode_1, param_mode_2, param_mode_3),
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

impl From<i32> for ParamMode {
    fn from(mode: i32) -> Self {
        match mode {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Invalid ParamMode"),
        }
    }
}

fn parse_instruction(instruction: i32) -> (i32, ParamMode, ParamMode, ParamMode) {
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
    program: Vec<i32>,
    instruction_pointer: usize,
    relative_pointer: usize,
    current_opcode: Option<Opcode>,
    current_params: [i32; 3],
    inputs: VecDeque<i32>,
    jumped: bool,
}

impl IntcodeVM {
    pub fn new() -> Self {
        IntcodeVM {
            program: vec![],
            instruction_pointer: 0,
            relative_pointer: 0,
            current_opcode: None,
            current_params: [0; 3],
            inputs: VecDeque::new(),
            jumped: false,
        }
    }

    pub fn set_program(&mut self, program: Vec<i32>) {
        self.program = program;
    }

    pub fn reset(&mut self) {
        self.program.clear();
        self.instruction_pointer = 0;
        self.relative_pointer = 0;
        self.current_opcode = None;
        self.current_params = [0; 3];
        self.inputs.clear();
        self.jumped = false;
    }

    pub fn push_input(&mut self, value: i32) {
        self.inputs.push_back(value);
    }

    fn parse_current_opcode(&self) -> Opcode {
        Opcode::from(self.program[self.instruction_pointer])
    }

    fn resolve_param(&mut self, param: i32, mode: ParamMode) -> i32 {
        match mode {
            ParamMode::Immediate => param,
            ParamMode::Position => self.program[param as usize],
            ParamMode::Relative => unimplemented!(),
        }
    }

    fn update_params(&mut self, modes: Vec<ParamMode>) {
        self.current_params = [0; 3];
        match self.current_opcode.unwrap() {
            Opcode::Add(..)
            | Opcode::Mul(..)
            | Opcode::JmpEZ(..)
            | Opcode::JmpNEZ(..)
            | Opcode::LessThen(..)
            | Opcode::Equals(..) => {
                for (i, mode) in modes.clone().into_iter().enumerate() {
                    self.current_params[i] =
                        self.resolve_param(self.program[self.instruction_pointer + i + 1], mode)
                }

                if modes.len() > 2 {
                    self.current_params[2] = self.program[self.instruction_pointer + 3];
                }
            }
            Opcode::Input(..) => {
                self.current_params[0] = self.program[self.instruction_pointer + 1]
            }
            Opcode::Output(..) => {
                self.current_params[0] =
                    self.resolve_param(self.program[self.instruction_pointer + 1], modes[0])
            }
            Opcode::Halt => {}
            Opcode::Invalid => panic!("Unsupported opcode!"),
        }
    }

    fn advance_parser(&mut self) {
        if !self.jumped {
            self.instruction_pointer += match self.current_opcode.unwrap() {
                Opcode::Add(..) | Opcode::Mul(..) | Opcode::LessThen(..) | Opcode::Equals(..) => 4,
                Opcode::Input(..) | Opcode::Output(..) => 2,
                Opcode::JmpEZ(..) | Opcode::JmpNEZ(..) => 3,
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
                Opcode::Add(mode1, mode2, mode3) => {
                    self.update_params(vec![mode1, mode2, mode3]);
                    self.program[self.current_params[2] as usize] =
                        self.current_params[0] + self.current_params[1];
                }
                Opcode::Mul(mode1, mode2, mode3) => {
                    self.update_params(vec![mode1, mode2, mode3]);
                    self.program[self.current_params[2] as usize] =
                        self.current_params[0] * self.current_params[1];
                }
                Opcode::Input(mode) => {
                    self.update_params(vec![mode]);
                    self.program[self.current_params[0] as usize] = match self.inputs.pop_front() {
                        Some(x) => x,
                        None => return VMState::AwaitsInput,
                    };
                }
                Opcode::Output(mode) => {
                    self.update_params(vec![mode]);
                    output = Some(self.current_params[0]);
                }
                Opcode::JmpEZ(mode1, mode2) => {
                    self.update_params(vec![mode1, mode2]);
                    if self.current_params[0] == 0 {
                        self.instruction_pointer = self.current_params[1] as usize;
                        self.jumped = true;
                    }
                }
                Opcode::JmpNEZ(mode1, mode2) => {
                    self.update_params(vec![mode1, mode2]);
                    if self.current_params[0] != 0 {
                        self.instruction_pointer = self.current_params[1] as usize;
                        self.jumped = true;
                    }
                }
                Opcode::LessThen(mode1, mode2, mode3) => {
                    self.update_params(vec![mode1, mode2, mode3]);
                    self.program[self.current_params[2] as usize] =
                        if self.current_params[0] < self.current_params[1] {
                            1
                        } else {
                            0
                        }
                }
                Opcode::Equals(mode1, mode2, mode3) => {
                    self.update_params(vec![mode1, mode2, mode3]);
                    self.program[self.current_params[2] as usize] =
                        if self.current_params[0] == self.current_params[1] {
                            1
                        } else {
                            0
                        }
                }
                Opcode::Halt => return VMState::Halted,
                Opcode::Invalid => panic!(
                    "Invalid Opcode! Previous State: Opcode = {:?}, Params = {:?}",
                    self.previous_opcode, self.previous_params
                ),
            }

            println!(
                "State: Opcode = {:?}, Params = {:?}, Params RAW: {:?}",
                self.current_opcode,
                self.current_params,
                self.program[((self.instruction_pointer + 1)
                    ..=(self.instruction_pointer + 3).min(self.program.len() - 1))]
                    .to_vec()
            );
            println!("Program: {:?}\n", &self.program[0..20]);

            self.advance_parser();
        }
    }

    pub fn run_once(&mut self) -> Vec<i32> {
        let mut output = vec![];

        loop {
            match self.execute() {
                VMState::Halted => break,
                VMState::AwaitsInput => panic!("Not enough input"),
                VMState::Output(x) => output.push(x),
            }
        }

        output
    }
}
