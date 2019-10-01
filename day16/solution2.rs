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

#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
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
    for i in 0..16 {
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

fn get_possible_ops(rc: &RegChange) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::new();
    output.push(rc.before[rc.instruction[1] as usize] + rc.before[rc.instruction[2] as usize] == rc.after[rc.instruction[3] as usize]); // addr
    output.push(rc.before[rc.instruction[1] as usize] + rc.instruction[2] == rc.after[rc.instruction[3] as usize]); // addi
    output.push(rc.before[rc.instruction[1] as usize] * rc.before[rc.instruction[2] as usize] == rc.after[rc.instruction[3] as usize]); // mulr
    output.push(rc.before[rc.instruction[1] as usize] * rc.instruction[2] == rc.after[rc.instruction[3] as usize]); // muli
    output.push(rc.before[rc.instruction[1] as usize] & rc.before[rc.instruction[2] as usize] == rc.after[rc.instruction[3] as usize]); // banr
    output.push(rc.before[rc.instruction[1] as usize] & rc.instruction[2] == rc.after[rc.instruction[3] as usize]); // bani
    output.push(rc.before[rc.instruction[1] as usize] | rc.before[rc.instruction[2] as usize] == rc.after[rc.instruction[3] as usize]); // borr
    output.push(rc.before[rc.instruction[1] as usize] | rc.instruction[2] == rc.after[rc.instruction[3] as usize]); // bori
    output.push(rc.before[rc.instruction[1] as usize] == rc.after[rc.instruction[3] as usize]); // setr
    output.push(rc.instruction[1] == rc.after[rc.instruction[3] as usize]); // seti
    output.push(rc.instruction[1] > rc.before[rc.instruction[2] as usize] && rc.after[rc.instruction[3] as usize] == 1); // gtir
    output.push(rc.before[rc.instruction[1] as usize] > rc.instruction[2] && rc.after[rc.instruction[3] as usize] == 1); // gtri
    output.push(rc.before[rc.instruction[1] as usize] > rc.before[rc.instruction[2] as usize] && rc.after[rc.instruction[3] as usize] == 1); // gtrr
    output.push(rc.instruction[1] == rc.before[rc.instruction[2] as usize] && rc.after[rc.instruction[3] as usize] == 1); // eqir
    output.push(rc.before[rc.instruction[1] as usize] == rc.instruction[2] && rc.after[rc.instruction[3] as usize] == 1); // eqri
    output.push(rc.before[rc.instruction[1] as usize] == rc.before[rc.instruction[2] as usize] && rc.after[rc.instruction[3] as usize] == 1); // eqrr
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

    //println!("reg_events: {:?}, example_program: {}", reg_events.len(), example_program.len());
    (reg_events, example_program)
}

fn reduce_op_codes(reg_events: Vec<RegChange>, mut uk_op_list: Vec<HashSet<Op>>, op_list: Vec<Op>) -> Vec<Op> {
    for event in reg_events.iter() {
        let inst_index = event.instruction[0] as usize;
        let possible_ops = get_possible_ops(event); // Seems correct up to here (unless I messed up ordering)
        for (i, po) in possible_ops.iter().enumerate() {
            if !po {
                // remove that op from the options
                if uk_op_list[inst_index].contains(&op_list[i]) {
                    uk_op_list[inst_index].remove(&op_list[i]);
                }
            }
        }
    }

    for (j, val) in uk_op_list.iter().enumerate() {
        println!("i: {}, v: {:?}", j, val);
    }

    let mut count = 0;
    loop {
        for i in 0..16 {
            if uk_op_list[i].len() == 1 {
                let mut removal: Op = Op::addr;
                for uk in uk_op_list[i].drain() {
                    removal = uk;
                }
                //println!("it: {:?}, it: {:?}", uk_op_list[i], op_list[i]);
                for (j, val) in uk_op_list.iter_mut().enumerate() {
                    if j != i {
                        //println!("Removed: {:?}", val.remove(&op_list[i]));
                        if val.len() > 1 {
                        val.remove(&removal);
                        }
                    }
                }
                uk_op_list[i].insert(removal);
            }
        }
                //println!("map: {:?}", uk_op_list);
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

fn operate(reg: &mut Vec<i32>, op: &Vec<i32>, op_codes: &Vec<Op>) {
    match op_codes[op[0] as usize] {
        Op::addr => { reg[op[3] as usize] = reg[op[1] as usize] + reg[op[2] as usize]; },
        Op::addi => { reg[op[3] as usize] = reg[op[1] as usize] + op[2]; },
        Op::mulr => { reg[op[3] as usize] = reg[op[1] as usize] * reg[op[2] as usize]; },
        Op::muli => { reg[op[3] as usize] = reg[op[1] as usize] * op[2]; },
        Op::banr => { reg[op[3] as usize] = reg[op[1] as usize] & reg[op[2] as usize]; },
        Op::bani => { reg[op[3] as usize] = reg[op[1] as usize] & op[2]; },
        Op::borr => { reg[op[3] as usize] = reg[op[1] as usize] | reg[op[2] as usize]; },
        Op::bori => { reg[op[3] as usize] = reg[op[1] as usize] | op[2]; },
        Op::setr => { reg[op[3] as usize] = reg[op[1] as usize]; },
        Op::seti => { reg[op[3] as usize] = op[1]; },
        Op::gtir => { if op[1] > reg[op[2] as usize] { reg[op[3] as usize] = 1; } else { reg[op[3] as usize] = 0; } },
        Op::gtri => { if reg[op[1] as usize] > op[2] { reg[op[3] as usize] = 1; } else { reg[op[3] as usize] = 0; } },
        Op::gtrr => { if reg[op[1] as usize] > reg[op[2] as usize] { reg[op[3] as usize] = 1; } else { reg[op[3] as usize] = 0; } },
        Op::eqir => { if op[1] == reg[op[2] as usize] { reg[op[3] as usize] = 1; } else { reg[op[3] as usize] = 0; } },
        Op::eqri => { if reg[op[1] as usize] == op[2] { reg[op[3] as usize] = 1; } else { reg[op[3] as usize] = 0; } },
        Op::eqrr => { if reg[op[1] as usize] == reg[op[2] as usize] { reg[op[3] as usize] = 1; } else { reg[op[3] as usize] = 0; } },
    };
}

fn run_program(program: Vec<Vec<i32>>, op_codes: Vec<Op>) -> i32 {
    let mut reg = vec![0;4];
    for op in program.iter() {
        operate(&mut reg, op, &op_codes);
        println!("reg: {:?}, op: {:?}, op_code: {:?}", reg, op, op_codes[op[0] as usize]);
    }
    reg[0]
}

fn main() {
    let (reg_events, example_program) = parse_input("input.txt");
    //let (reg_events, _example_program) = parse_input("input.test.txt");
    let uk_op_list = generate_unknown_op_list();
    let op_list = generate_op_list();
    //println!("uk_op_list: {:?}, op_list: {:?}", uk_op_list, op_list);
    let op_codes = reduce_op_codes(reg_events, uk_op_list, op_list);
    //println!("Three or more op code count: {}", output.iter().map(|r| if r.len() >= 3 { 1 } else { 0 }).fold(0, |a,b| a+b));
    println!("Op codes: {:?}", op_codes);
    let output = run_program(example_program, op_codes);
    println!("{:?}", output);
}

