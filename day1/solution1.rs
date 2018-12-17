use std::fs;
use std::io;
use std::process;
use std::io::BufRead;

fn main() {
    let mut frequency: i32 = 0;
    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    for mut line in reader.lines() {
        let mut line_str = line.unwrap();

        let sign: char = line_str.remove(0);
        if sign == '+' {
            let parsed_number: i32 = line_str.parse().unwrap();
            frequency += parsed_number;
        } else if sign == '-' {
            let parsed_number: i32 = line_str.parse().unwrap();
            frequency -= parsed_number;
        } else {
            println!("ERROR: We enfrequencyed an input that doesn't have a +/- as the first char: {}", sign);
            process::exit(1);
        }
    }

    println!("Final frequency: {}", frequency);
}

