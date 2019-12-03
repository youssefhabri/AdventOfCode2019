fn main() {
    let input = include_str!("../../inputs/day02.txt");
    let mut opcodes = input
        .split(',')
        .enumerate()
        .map(|(i, opcode)| {
            opcode
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Valid int at {}", i + 1))
        })
        .collect::<Vec<u32>>();

    let mut noun = 0;
    let mut verb = 0;

    'outer: for n in 0..=99 {
        for v in 0..=99 {
            opcodes[1] = n;
            opcodes[2] = v;

            let result = parse(opcodes.clone());
            if result == 19_690_720 {
                noun = n;
                verb = v;
                break 'outer;
            }
        }
    }

    println!("{}", 100 * noun + verb);
}

fn parse(mut opcodes: Vec<u32>) -> u32 {
    let mut ip = 0;
    loop {
        let a_addr = opcodes[ip + 1] as usize;
        let b_addr = opcodes[ip + 2] as usize;
        let r_addr = opcodes[ip + 3] as usize;

        match opcodes[ip] {
            1 => {
                opcodes[r_addr] = opcodes[a_addr] + opcodes[b_addr];
                ip += 4;
            }
            2 => {
                opcodes[r_addr] = opcodes[a_addr] * opcodes[b_addr];
                ip += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode at {}", ip),
        }
    }

    opcodes[0]
}
