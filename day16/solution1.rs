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

fn main() {
    // Create instruction mapping using bit flags start at 16-1 (1<<4 - 1) for all flags
    let (reg_events, example_program) = parse_input("input.txt");
}

