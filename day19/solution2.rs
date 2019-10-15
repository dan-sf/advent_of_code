use std::fs;
use std::io;
use std::io::BufRead;


#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Op {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

fn operate(reg: &mut Vec<i32>, inst: &(Op, i32, i32, i32)) {
    match inst.0 {
        Op::addr => { reg[inst.3 as usize] = reg[inst.1 as usize] + reg[inst.2 as usize]; },
        Op::addi => { reg[inst.3 as usize] = reg[inst.1 as usize] + inst.2; },
        Op::mulr => { reg[inst.3 as usize] = reg[inst.1 as usize] * reg[inst.2 as usize]; },
        Op::muli => { reg[inst.3 as usize] = reg[inst.1 as usize] * inst.2; },
        Op::banr => { reg[inst.3 as usize] = reg[inst.1 as usize] & reg[inst.2 as usize]; },
        Op::bani => { reg[inst.3 as usize] = reg[inst.1 as usize] & inst.2; },
        Op::borr => { reg[inst.3 as usize] = reg[inst.1 as usize] | reg[inst.2 as usize]; },
        Op::bori => { reg[inst.3 as usize] = reg[inst.1 as usize] | inst.2; },
        Op::setr => { reg[inst.3 as usize] = reg[inst.1 as usize]; },
        Op::seti => { reg[inst.3 as usize] = inst.1; },
        Op::gtir => { if inst.1 > reg[inst.2 as usize] { reg[inst.3 as usize] = 1; } else { reg[inst.3 as usize] = 0; } },
        Op::gtri => { if reg[inst.1 as usize] > inst.2 { reg[inst.3 as usize] = 1; } else { reg[inst.3 as usize] = 0; } },
        Op::gtrr => { if reg[inst.1 as usize] > reg[inst.2 as usize] { reg[inst.3 as usize] = 1; } else { reg[inst.3 as usize] = 0; } },
        Op::eqir => { if inst.1 == reg[inst.2 as usize] { reg[inst.3 as usize] = 1; } else { reg[inst.3 as usize] = 0; } },
        Op::eqri => { if reg[inst.1 as usize] == inst.2 { reg[inst.3 as usize] = 1; } else { reg[inst.3 as usize] = 0; } },
        Op::eqrr => { if reg[inst.1 as usize] == reg[inst.2 as usize] { reg[inst.3 as usize] = 1; } else { reg[inst.3 as usize] = 0; } },
    };
}

fn parse_input(path: &str) -> (usize, Vec<(Op, i32, i32, i32)>) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
    let mut ip_reg: usize = 0;
    let mut program: Vec<(Op, i32, i32, i32)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let split_line = line.split(' ').map(|r| r.to_string()).collect::<Vec<String>>();
        if i == 0 {
            ip_reg = split_line[1].parse::<usize>().unwrap();
            continue;
        }

        let inst = match split_line[0].as_str() {
            "addr" => { Op::addr },
            "addi" => { Op::addi },
            "mulr" => { Op::mulr },
            "muli" => { Op::muli },
            "banr" => { Op::banr },
            "bani" => { Op::bani },
            "borr" => { Op::borr },
            "bori" => { Op::bori },
            "setr" => { Op::setr },
            "seti" => { Op::seti },
            "gtir" => { Op::gtir },
            "gtri" => { Op::gtri },
            "gtrr" => { Op::gtrr },
            "eqir" => { Op::eqir },
            "eqri" => { Op::eqri },
            "eqrr" => { Op::eqrr },
            _ => { panic!("Error unexpected instruction") },
        };

        let parsed_reg: Vec<i32> = split_line[1..].iter().map(|r| r.parse::<i32>().unwrap()).collect();

        program.push((inst, parsed_reg[0], parsed_reg[1], parsed_reg[2]));
    }

    (ip_reg, program)
}

fn run_program(program: Vec<(Op, i32, i32, i32)>, ip_reg: usize) -> i32 {
    let mut ip = 0;
    let mut register: Vec<i32> = vec![0;6];
    register[0] = 1;

    loop {
        //print!("ip={} {:?} {:?} ", ip, register, program[ip as usize]);
        //print!("{:?} ", register);
        // Write the ip to the ip_reg
        register[ip_reg] = ip;
        // Execute instruction
        operate(&mut register, &program[ip as usize]);
        //println!("{:?}", register);
        // Write the reg back to the ip
        ip = register[ip_reg];
        // Increase the ip by 1
        ip += 1;

        if ip as usize >= program.len() {
            break;
        }
        //println!("{}", register[0]);
    }

    register[0]
}

fn main() {
    let (ip_reg, program) = parse_input("input.txt");
    //let (ip_reg, program) = parse_input("input.test.txt");
    let result = run_program(program, ip_reg);
    println!("Result: {:?}", result);
}

