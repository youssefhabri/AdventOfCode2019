use std::io;

fn main() {
    let input = include_str!("../../inputs/day05.txt");
    let opcodes = input
        .split(',')
        .enumerate()
        .map(|(i, opcode)| {
            opcode
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Valid int at {}", i + 1))
        })
        .collect::<Vec<i32>>();

    //    dbg!(opcodes.len());

    let _ = parse(opcodes, vec![]);
}

fn parse(mut opcodes: Vec<i32>, inputs: Vec<i32>) -> i32 {
    let mut instruction_pointer = 0;
    let mut input_counter = 0;
    let mut output = 0;
    loop {
        println!(
            "[DEBUG] START EXEC OPCODE #{}, {}",
            instruction_pointer, opcodes[instruction_pointer]
        );
        let (instruction, a_addr_mode, b_addr_mode, _r_addr_mode) =
            parse_instruction(opcodes[instruction_pointer]);
        println!("[DEBUG] INSTRUCTION: {}", instruction);

        let params = || {
            let param_a = load_memory(&opcodes, instruction_pointer + 1, a_addr_mode);
            let param_b = load_memory(&opcodes, instruction_pointer + 2, b_addr_mode);
            let param_c = opcodes[instruction_pointer + 3] as usize;
            (param_a, param_b, param_c)
        };

        match instruction {
            1 => {
                let (param_a, param_b, output_addr) = params();

                opcodes[output_addr] = param_a + param_b;
                instruction_pointer += 4;
            }
            2 => {
                let (param_a, param_b, output_addr) = params();

                opcodes[output_addr] = param_a * param_b;
                instruction_pointer += 4;
            }
            3 => {
                let value = if inputs.is_empty() {
                    let mut input = String::new();
                    let _ = io::stdin().read_line(&mut input);
                    dbg!(&input);
                    input.trim().parse::<i32>().unwrap()
                } else {
                    let value = inputs[input_counter];
                    input_counter += 1;

                    value
                };
                let addr = opcodes[instruction_pointer + 1] as usize;
                opcodes[addr] = value;
                instruction_pointer += 2;
            }
            4 => {
                let addr = opcodes[instruction_pointer + 1] as usize;
                output = opcodes[addr];
                println!("[OUTPUT] {}", output);
                instruction_pointer += 2;
            }
            5 => {
                let (param_a, param_b, _) = params();

                println!("[DEBUG] [JMP-NZ] IN_A: {}, IN_B: {}", param_a, param_b);

                if param_a != 0 {
                    instruction_pointer = param_b as usize
                } else {
                    instruction_pointer += 3;
                }
            }
            6 => {
                let (param_a, param_b, _) = params();

                if param_a == 0 {
                    instruction_pointer = param_b as usize
                } else {
                    instruction_pointer += 3;
                }
            }
            7 => {
                let (param_a, param_b, output_addr) = params();

                opcodes[output_addr] = if param_a < param_b { 1 } else { 0 };
                instruction_pointer += 4;
            }
            8 => {
                let (param_a, param_b, output_addr) = params();

                opcodes[output_addr] = if param_a == param_b { 1 } else { 0 };
                instruction_pointer += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode at {}", instruction_pointer),
        }
    }

    output
}

fn parse_instruction(opcode: i32) -> (i32, i32, i32, i32) {
    let instruction = opcode % 100;
    let a_addr_mode = (opcode / 100) % 10;
    let b_addr_mode = (opcode / 1000) % 10;
    let r_addr_mode = (opcode / 10000) % 10;

    (instruction, a_addr_mode, b_addr_mode, r_addr_mode)
}

fn load_memory(opcodes: &Vec<i32>, ip: usize, mode: i32) -> i32 {
    match mode {
        0 => {
            let addr = opcodes[ip];
            opcodes[addr as usize]
        }
        1 => opcodes[ip],
        _ => unreachable!(),
    }
}
