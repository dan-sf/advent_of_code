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

fn operate(reg: &mut Vec<u64>, inst: &(Op, u64, u64, u64)) {
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

fn parse_input(path: &str) -> (usize, Vec<(Op, u64, u64, u64)>) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
    let mut ip_reg: usize = 0;
    let mut program: Vec<(Op, u64, u64, u64)> = Vec::new();

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

        let parsed_reg: Vec<u64> = split_line[1..].iter().map(|r| r.parse::<u64>().unwrap()).collect();

        program.push((inst, parsed_reg[0], parsed_reg[1], parsed_reg[2]));
    }

    (ip_reg, program)
}

// Decompile function to make reading the asm easier
fn _decompile_program(program: &Vec<(Op, u64, u64, u64)>, ip_reg: usize) {
    let vars = ['a', 'b', 'c', 'd', 'e', 'f'];
    for (i, inst) in program.iter().enumerate() {

        if i < 10 {
            print!("{}  ", i);
        } else {
            print!("{} ", i);
        }

        match inst.0 {
            Op::addr => { print!("{} = {} + {}", vars[inst.3 as usize], vars[inst.1 as usize], vars[inst.2 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::addi => { print!("{} = {} + {}", vars[inst.3 as usize], vars[inst.1 as usize], inst.2);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::mulr => { print!("{} = {} * {}", vars[inst.3 as usize], vars[inst.1 as usize], vars[inst.2 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::muli => { print!("{} = {} * {}", vars[inst.3 as usize], vars[inst.1 as usize], inst.2);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::banr => { print!("{} = {} & {}", vars[inst.3 as usize], vars[inst.1 as usize], vars[inst.2 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::bani => { print!("{} = {} & {}", vars[inst.3 as usize], vars[inst.1 as usize], inst.2);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::borr => { print!("{} = {} | {}", vars[inst.3 as usize], vars[inst.1 as usize], vars[inst.2 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::bori => { print!("{} = {} | {}", vars[inst.3 as usize], vars[inst.1 as usize], inst.2);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::setr => { print!("{} = {}", vars[inst.3 as usize], vars[inst.1 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::seti => { print!("{} = {}", vars[inst.3 as usize], inst.1);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::gtir => { print!("if {} > {} {{ {} = 1; }} else {{ {} = 0; }}", inst.1, vars[inst.2 as usize], vars[inst.3 as usize], vars[inst.3 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::gtri => { print!("if {} > {} {{ {} = 1; }} else {{ {} = 0; }}", vars[inst.1 as usize], inst.2, vars[inst.3 as usize], vars[inst.3 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::gtrr => { print!("if {} > {} {{ {} = 1; }} else {{ {} = 0; }}", vars[inst.1 as usize], vars[inst.2 as usize], vars[inst.3 as usize], vars[inst.3 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::eqir => { print!("if {} == {} {{ {} = 1; }} else {{ {} = 0; }}", inst.1, vars[inst.2 as usize], vars[inst.3 as usize], vars[inst.3 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::eqri => { print!("if {} == {} {{ {} = 1; }} else {{ {} = 0; }}", vars[inst.1 as usize], inst.2, vars[inst.3 as usize], vars[inst.3 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
            Op::eqrr => { print!("if {} == {} {{ {} = 1; }} else {{ {} = 0; }}", vars[inst.1 as usize], vars[inst.2 as usize], vars[inst.3 as usize], vars[inst.3 as usize]);
                if inst.3 as usize == ip_reg { println!("; GOTO {}", vars[inst.3 as usize]); } else { println!(); } },
        };
    }
}

fn run_program(program: Vec<(Op, u64, u64, u64)>, ip_reg: usize) -> u64 {
    let mut ip = 0;
    let mut register: Vec<u64> = vec![0;6];
    register[0] = 10846352; // Found experimentally

    loop {

        // if ip == 28 {
        //     println!("{}, {:?}", register[5], register);
        // }

        // Write the ip to the ip_reg
        register[ip_reg] = ip;
        // Execute instruction
        operate(&mut register, &program[ip as usize]);

        // Write the reg back to the ip
        ip = register[ip_reg];
        // Increase the ip by 1
        ip += 1;

        if ip as usize >= program.len() {
            break;
        }
    }

    register[0]
}

// For this problem I just decompiled the program and noticed that it will halt when register 0 is
// equal to register 5 at instruction 28. So I just ran the program without setting reg 0 and took
// note of what reg 5 was at that point and grabbed the first number. (This solution is most likely
// specific to my input)

fn main() {
    let (ip_reg, program) = parse_input("input.txt");
    //_decompile_program(&program, ip_reg);
    let result = run_program(program, ip_reg);
    println!("Result: {:?}", result);
}

