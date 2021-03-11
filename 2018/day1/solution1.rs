use std::io;
use std::io::BufRead;

fn main() {
    let mut frequency: i32 = 0;
    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    for line in reader.lines() {
        frequency += line.unwrap().parse::<i32>().unwrap();
    }

    println!("Final frequency: {}", frequency);
}

