use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn parse_input(path: &str) -> (String, HashMap<String, char>) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    // Parse out initial state and notes
    let mut line_number = 1;
    let mut state = String::from("");
    let mut notes = HashMap::new();
    for line in reader.lines() {
        if line_number == 1 {
            state = line.unwrap().replace("initial state: ", "");
        } else if line_number > 2 {
            let parsed_line: Vec<String> = line.unwrap()
                .split(" => ").map(|l| l.to_string()).collect();
            let key = parsed_line[0].clone();
            let value: char = parsed_line[1].clone().as_bytes()[0] as char;
            notes.insert(key, value);
        }
        line_number += 1;
    }
    (state, notes)
}

fn get_gen_counts(state: String, notes: HashMap<String, char>) -> i32 {
    let mut output = 0;
    let mut cur_state = state.clone();

    let mut center = 0;
    for _gen in 1..=20 {
        let mut ch_array: Vec<char> = cur_state.as_str().chars().collect();
        let mut gen_state = String::new();

        // Expand the array if we have pots close to the beginning or end
        if ch_array[0] == '#' {
            ch_array.insert(0, '.'); ch_array.insert(0, '.');
            center += 2;
        } else if ch_array[1] == '#' {
            ch_array.insert(0, '.');
            center += 1;
        }

        if ch_array[ch_array.len()-1] == '#' {
            ch_array.push('.'); ch_array.push('.');
        } else if ch_array[ch_array.len()-2] == '#' {
            ch_array.push('.');
        }

        for pot in 0..ch_array.len() {
            let mut check: String;
            if pot == 0 {
                check = String::from("..");
                check.push(ch_array[pot]);
                check.push(ch_array[pot+1]);
                check.push(ch_array[pot+2]);
                gen_state.push(notes[&check]);
            } else if pot == 1 {
                check = String::from(".");
                check.push(ch_array[pot-1]);
                check.push(ch_array[pot]);
                check.push(ch_array[pot+1]);
                check.push(ch_array[pot+2]);
                gen_state.push(notes[&check]);
            } else if pot == ch_array.len()-1 {
                check = String::from("");
                check.push(ch_array[pot-2]);
                check.push(ch_array[pot-1]);
                check.push(ch_array[pot]);
                check.push('.');
                check.push('.');
                gen_state.push(notes[&check]);
            } else if pot == ch_array.len()-2 {
                check = String::from("");
                check.push(ch_array[pot-2]);
                check.push(ch_array[pot-1]);
                check.push(ch_array[pot]);
                check.push(ch_array[pot+1]);
                check.push('.');
                gen_state.push(notes[&check]);
            } else {
                check = String::from("");
                check.push(ch_array[pot-2]);
                check.push(ch_array[pot-1]);
                check.push(ch_array[pot]);
                check.push(ch_array[pot+1]);
                check.push(ch_array[pot+2]);
                gen_state.push(notes[&check]);
            }
        }
        cur_state = gen_state;
    }
    for (i, ch) in cur_state.chars().enumerate() {
        if ch == '#' {
            output += (i as i32)-center;
        }
    }
    output
}

fn main() {
    let (state, notes) = parse_input("input.txt");
    println!("Number of pots: {}", get_gen_counts(state, notes));
}

