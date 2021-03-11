use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct RegChange {
    before: Vec<i32>,
    after: Vec<i32>,
    instruction: Vec<i32>,
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

    (reg_events, example_program)
}

fn get_many_op_sample_count(reg_events: Vec<RegChange>) -> i32 {
    let mut output = 0;
    for event in reg_events.iter() {
        let possible_ops = get_possible_ops(event);
        let count = possible_ops.iter().map(|r| if *r { 1 } else { 0 }).fold(0, |a,b| a+b);
        if count >= 3 {
            output += 1;
        }
    }
    return output
}

fn main() {
    let (reg_events, _example_program) = parse_input("input.txt");
    let output = get_many_op_sample_count(reg_events);
    println!("Three or more op code count: {}", output);
}

