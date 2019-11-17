use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;


fn parse_input(path: &str) -> Vec<Vec<i32>> {
    let input = File::open(path)
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(input);

    let mut output: Vec<Vec<i32>> = vec![vec![]];
    for line in reader.lines() {
        output.push(line.unwrap().split(",").map(|r| r.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    }

    output
}

fn main() {
    let points = parse_input("input.txt");
    println!("{:?}", points);
}

