use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn get_count(s: String) -> HashMap<char, u32> {
    let mut counter = HashMap::new();
    for c in s.chars() {
        let value = counter.entry(c).or_insert(0);
        *value += 1;
    }
    counter
}

fn main() {
    let mut two = 0;
    let mut three = 0;

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    for line in reader.lines() {
        let c = get_count(line.unwrap());
        if c.values().any(|x| *x == 2) {
            two += 1;
        }
        if c.values().any(|x| *x == 3) {
            three += 1;
        }
    }
    println!("Input checksum: {}", two*three);
}

