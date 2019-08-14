use std::fs;
use std::i32::MAX;
use std::io::Read;
use std::collections::HashSet;

//fn reduce_poly(polymer: &String) -> i32 {
fn reduce_poly(poly_chars: &mut Vec<char>) -> i32 {
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
    poly_chars.len() as i32
}

fn remove_char(poly_chars: &mut Vec<char>, ch: char) {
    let mut index = 0;
    while index < poly_chars.len() {
        let lower = poly_chars[index].to_lowercase().collect::<Vec<char>>()[0];
        if lower == ch {
            poly_chars.remove(index);
        } else {
            index += 1;
        }
    }
}

fn main() {
    let mut input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");

    let mut polymer = String::new();
    input.read_to_string(&mut polymer).unwrap();

    let mut ch_set = HashSet::new();
    for ch in polymer.chars() {
        ch_set.insert(ch.to_lowercase().collect::<Vec<char>>()[0]);
    }

    let mut smallest = MAX;
    for ch in &ch_set {
        let mut poly_chars: Vec<char> = polymer.trim_end().chars().collect();
        remove_char(&mut poly_chars, *ch);
        let reduced_size = reduce_poly(&mut poly_chars);
        if reduced_size < smallest {
            smallest = reduced_size;
        }
    }

    println!("Number of units in the reduced polymer: {}", smallest);
}

