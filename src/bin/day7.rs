use advent_of_code::{IntcodeVM, VMState};

fn main() {
    let input = include_str!("../../inputs/day07.txt");
    let opcodes = input
        .split(',')
        .enumerate()
        .map(|(i, opcode)| {
            opcode
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Valid int at {}", i + 1))
        })
        .collect::<Vec<i32>>();

    let mut amps = vec![IntcodeVM::new(); 5];

    let settings_list = generate_combinations(5, 9);

    let mut highest_output = 0;

    for settings in settings_list {
        for (i, amp) in amps.iter_mut().enumerate() {
            amp.reset();
            amp.set_program(opcodes.clone());
            amp.push_input(settings[i]);
        }

        let mut index = 0;
        let mut prev_output = 0;

        'outer: loop {
            loop {
                match amps[index].execute() {
                    VMState::Halted => {
                        if prev_output > highest_output {
                            highest_output = prev_output;
                        }
                        println!("Settings: {:?}; Output: {}", settings, prev_output);
                        break 'outer;
                    }
                    VMState::AwaitsInput => amps[index].push_input(prev_output),
                    VMState::Output(output) => {
                        prev_output = output;
                        break;
                    }
                }
            }

            index = if index == 4 { 0 } else { index + 1 };
        }
    }

    print!("{}", highest_output);
}

fn generate_combinations(start: i32, end: i32) -> Vec<Vec<i32>> {
    let mut combinations = vec![];
    let range = (start..(end + 1)).collect::<Vec<i32>>();
    for a in range.clone() {
        for b in range.clone().into_iter().filter(|i| *i != a) {
            for c in range.clone().into_iter().filter(|i| *i != a && *i != b) {
                for d in range
                    .clone()
                    .into_iter()
                    .filter(|i| *i != a && *i != b && *i != c)
                {
                    for e in range
                        .clone()
                        .into_iter()
                        .filter(|i| *i != a && *i != b && *i != c && *i != d)
                    {
                        combinations.push(vec![a, b, c, d, e])
                    }
                }
            }
        }
    }

    combinations
}
