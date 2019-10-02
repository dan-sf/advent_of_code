use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

#[derive(Debug)]
struct RegChange {
    before: Vec<i32>,
    after: Vec<i32>,
    instruction: Vec<i32>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

fn generate_unknown_op_list() -> Vec<HashSet<Op>> {
    let mut output: Vec<HashSet<Op>> = Vec::new();
    for _ in 0..16 {
        let mut operations: HashSet<Op> = HashSet::new();
        operations.insert(Op::addr);
        operations.insert(Op::addi);
        operations.insert(Op::mulr);
        operations.insert(Op::muli);
        operations.insert(Op::banr);
        operations.insert(Op::bani);
        operations.insert(Op::borr);
        operations.insert(Op::bori);
        operations.insert(Op::setr);
        operations.insert(Op::seti);
        operations.insert(Op::gtir);
        operations.insert(Op::gtri);
        operations.insert(Op::gtrr);
        operations.insert(Op::eqir);
        operations.insert(Op::eqri);
        operations.insert(Op::eqrr);
        output.push(operations);
    }
    output
}

fn generate_op_list() -> Vec<Op> {
    let mut output: Vec<Op> = Vec::new();
    output.push(Op::addr);
    output.push(Op::addi);
    output.push(Op::mulr);
    output.push(Op::muli);
    output.push(Op::banr);
    output.push(Op::bani);
    output.push(Op::borr);
    output.push(Op::bori);
    output.push(Op::setr);
    output.push(Op::seti);
    output.push(Op::gtir);
    output.push(Op::gtri);
    output.push(Op::gtrr);
    output.push(Op::eqir);
    output.push(Op::eqri);
    output.push(Op::eqrr);
    output
}

// Given a register change, output which ops are valid for that change
fn get_possible_ops(rc: &RegChange, op_codes: &Vec<Op>) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::new();
    for i in 0..16 {
        let mut test_op = rc.before.clone();
        operate(&mut test_op, &rc.instruction, &op_codes[i]);
        output.push(rc.after == test_op);
    }
    output
}

fn parse_input(path: &str) -> (Vec<RegChange>, Vec<Vec<i32>>) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut reg_events: Vec<RegChange> = Vec::new();
    let mut example_program: Vec<Vec<i32>> = Vec::new();

    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
    let mut lines_iter = lines.iter();

    // Parse the first part of the input
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            break;
        }

        let before = &line.chars().collect::<Vec<char>>()[9..line.len()-1];
        let before = before.iter().filter(|c| !c.is_whitespace() && !(c == &&',')).collect::<Vec<&char>>();
        let before = before.iter().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let instruction = lines_iter.next().unwrap().split(' ').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let after = &lines_iter.next().unwrap().chars().collect::<Vec<char>>()[9..line.len()-1];
        let after = after.iter().filter(|c| !c.is_whitespace() && !(c == &&',')).collect::<Vec<&char>>();
        let after = after.iter().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let _empty = lines_iter.next().unwrap();

        reg_events.push(
            RegChange {
                before: before,
                instruction: instruction,
                after: after,
            });
    }

    // Parse the second part of the input
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            continue;
        }
        let instruction = line.split(' ').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        example_program.push(instruction);
    }

    (reg_events, example_program)
}

// Get what the actual op codes are mapped to
fn reduce_op_codes(reg_events: Vec<RegChange>, mut uk_op_list: Vec<HashSet<Op>>, op_list: Vec<Op>) -> Vec<Op> {
    for event in reg_events.iter() {
        let inst_index = event.instruction[0] as usize;
        let possible_ops = get_possible_ops(event, &op_list);
        for (i, po) in possible_ops.iter().enumerate() {
            if !po {
                // Remove that op from the options
                if uk_op_list[inst_index].contains(&op_list[i]) {
                    uk_op_list[inst_index].remove(&op_list[i]);
                }
            }
        }
    }

    // For ops that are now known, remove those as options for any other op codes we might think
    // they could be. Keep doing this until we have a single op code for each number
    loop {
        for i in 0..16 {
            if uk_op_list[i].len() == 1 {
                let removal: Op = uk_op_list[i].drain().collect::<Vec<Op>>().pop().unwrap();
                for j in 0..16 {
                    if j != i {
                        if uk_op_list[j].contains(&removal) {
                            uk_op_list[j].remove(&removal);
                        }
                    }
                }
                uk_op_list[i].insert(removal);
            }
        }

        // Break if we know all the op codes
        if uk_op_list.iter().all(|r| r.len() <= 1) {
            break;
        }
    }

    let mut output: Vec<Op> = Vec::new();
    for uk in uk_op_list.iter_mut() {
        output.push(uk.drain().collect::<Vec<Op>>().pop().unwrap());
    }
    return output;
}

fn operate(reg: &mut Vec<i32>, inst: &Vec<i32>, op: &Op) {
    match op {
        Op::addr => { reg[inst[3] as usize] = reg[inst[1] as usize] + reg[inst[2] as usize]; },
        Op::addi => { reg[inst[3] as usize] = reg[inst[1] as usize] + inst[2]; },
        Op::mulr => { reg[inst[3] as usize] = reg[inst[1] as usize] * reg[inst[2] as usize]; },
        Op::muli => { reg[inst[3] as usize] = reg[inst[1] as usize] * inst[2]; },
        Op::banr => { reg[inst[3] as usize] = reg[inst[1] as usize] & reg[inst[2] as usize]; },
        Op::bani => { reg[inst[3] as usize] = reg[inst[1] as usize] & inst[2]; },
        Op::borr => { reg[inst[3] as usize] = reg[inst[1] as usize] | reg[inst[2] as usize]; },
        Op::bori => { reg[inst[3] as usize] = reg[inst[1] as usize] | inst[2]; },
        Op::setr => { reg[inst[3] as usize] = reg[inst[1] as usize]; },
        Op::seti => { reg[inst[3] as usize] = inst[1]; },
        Op::gtir => { if inst[1] > reg[inst[2] as usize] { reg[inst[3] as usize] = 1; } else { reg[inst[3] as usize] = 0; } },
        Op::gtri => { if reg[inst[1] as usize] > inst[2] { reg[inst[3] as usize] = 1; } else { reg[inst[3] as usize] = 0; } },
        Op::gtrr => { if reg[inst[1] as usize] > reg[inst[2] as usize] { reg[inst[3] as usize] = 1; } else { reg[inst[3] as usize] = 0; } },
        Op::eqir => { if inst[1] == reg[inst[2] as usize] { reg[inst[3] as usize] = 1; } else { reg[inst[3] as usize] = 0; } },
        Op::eqri => { if reg[inst[1] as usize] == inst[2] { reg[inst[3] as usize] = 1; } else { reg[inst[3] as usize] = 0; } },
        Op::eqrr => { if reg[inst[1] as usize] == reg[inst[2] as usize] { reg[inst[3] as usize] = 1; } else { reg[inst[3] as usize] = 0; } },
    };
}

fn run_program(program: Vec<Vec<i32>>, op_codes: Vec<Op>) -> i32 {
    let mut reg = vec![0;4];
    for inst in program.iter() {
        operate(&mut reg, inst, &op_codes[inst[0] as usize]);
    }
    reg[0]
}

fn main() {
    let (reg_events, example_program) = parse_input("input.txt");
    let uk_op_list = generate_unknown_op_list();
    let op_list = generate_op_list();
    let op_codes = reduce_op_codes(reg_events, uk_op_list, op_list);
    let output = run_program(example_program, op_codes);
    println!("Register zero: {}", output);
}

