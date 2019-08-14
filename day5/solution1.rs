use std::fs;
use std::io::Read;

fn main() {
    let mut input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");

    let mut polymer = String::new();
    input.read_to_string(&mut polymer).unwrap();

    let mut poly_chars: Vec<char> = polymer.trim_end().chars().collect();
    let mut reductions = 1;
    let mut index;

    while reductions > 0 {
        reductions = 0;
        index = 0;
        while index < poly_chars.len() - 1 {
            let a_lower = poly_chars[index].to_lowercase().collect::<Vec<char>>()[0];
            let b_lower = poly_chars[index+1].to_lowercase().collect::<Vec<char>>()[0];
            if poly_chars[index] != poly_chars[index+1] && a_lower == b_lower {
                poly_chars.remove(index);
                poly_chars.remove(index);
                reductions += 1;
            }
            index += 1;
        }
    }

    println!("Number of units in the reduced polymer: {}", poly_chars.len());
}

