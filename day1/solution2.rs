use std::fs;
use std::io;
use std::io::BufRead;
use std::process;
use std::collections::HashSet;

fn main() {
    let mut check = HashSet::new();
    let mut frequency: i32 = 0;

    while !check.contains(&frequency) {
        let input = fs::File::open("input.txt")
            .expect("Something went wrong reading the file");
        let reader = io::BufReader::new(input);

        for line in reader.lines() {
            check.insert(frequency);
            frequency += line.unwrap().parse::<i32>().unwrap();
            if check.contains(&frequency) {
                println!("Final frequency: {}", frequency);
                process::exit(0);
            }
        }
    }
}

