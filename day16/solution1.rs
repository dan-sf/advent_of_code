use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
struct RegChange {
    before: Vec<i32>,
    after: Vec<i32>,
    instruction: Vec<i32>,
}

#[derive(Debug)]
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
    output: Vec<HashSet<Op>> = Vec::new();
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
    output: Vec<Op> = Vec::new();
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
    let mut output: Vec<bool> = vec![false;15];
    output.push(rc.before[1] + rc.before[2] == rc.after[3]); // addr
    output.push(rc.before[1] + rc.instruction[2] == rc.after[3]); // addi
    output.push(rc.before[1] * rc.before[2] == rc.after[3]); // mulr
    output.push(rc.before[1] * rc.instruction[2] == rc.after[3]); // muli
    output.push(rc.before[1] & rc.before[2] == rc.after[3]); // banr
    output.push(rc.before[1] & rc.instruction[2] == rc.after[3]); // bani
    output.push(rc.before[1] | rc.before[2] == rc.after[3]); // borr
    output.push(rc.before[1] | rc.instruction[2] == rc.after[3]); // bori
    output.push(rc.before[1] == rc.after[3]); // setr
    output.push(rc.instruction[1] == rc.after[3]); // seti
    output.push(rc.instruction[1] > rc.before[2] && rc.after[3] == 1); // gtir
    output.push(rc.before[1] > rc.instruction[2] && rc.after[3] == 1); // gtri
    output.push(rc.before[1] > rc.before[2] && rc.after[3] == 1); // gtrr
    output.push(rc.instruction[1] == rc.before[2] && rc.after[3] == 1); // eqir
    output.push(rc.before[1] == rc.instruction[2] && rc.after[3] == 1); // eqri
    output.push(rc.before[1] == rc.before[2] && rc.after[3] == 1); // eqrr
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

    println!("reg_events: {:?}, example_program: {}", reg_events.len(), example_program.len());
    (reg_events, example_program)
}

fn reduce_op_codes(reg_events: Vec<RegChange>, uk_op_list: Vec<HashSet<Op>>, op_list: Vec<Op>) -> Vec<Op> {
    for event in reg_events.iter() {
        let inst_index = event.instruction[0] as usize;
        let possible_ops = get_possible_ops(event);
        for (i, po) in possible_ops.enumerate() {
            if !po {
                // remove that op from the options
                if uk_op_list[inst_index].contains(op_list[i]) {
                    uk_op_list[inst_index].remove(op_list[i]);
                }
            }
        }
    }
    println!("{:?}", uk_op_list);
    return vec![Op::addr];

}

fn main() {
    // Create instruction mapping using bit flags start at 16-1 (1<<4 - 1) for all flags
    let (reg_events, example_program) = parse_input("input.txt");
}

