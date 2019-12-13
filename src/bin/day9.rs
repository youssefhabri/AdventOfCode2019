use advent_of_code::{IntcodeVM, VMState};

fn main() {
    let input = include_str!("../../inputs/day09.txt");
    let opcodes = input
        .split(',')
        .enumerate()
        .map(|(i, opcode)| {
            opcode
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("Not a valid int at {}", i + 1))
        })
        .collect::<Vec<i64>>();

    let mut vm = IntcodeVM::new();
    vm.set_program(opcodes);
    vm.push_input(1);

    loop {
        match vm.execute() {
            VMState::Halted => break,
            VMState::NeedsInput => panic!("Not enough input"),
            VMState::Output(output) => println!("{:?}", output),
        }
    }
}
