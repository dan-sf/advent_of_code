use std::fs;
use std::io;
use std::io::BufRead;

fn get_common_chars(box_one: &String, box_two: &String) -> String {
    box_one.chars()
        .zip(box_two.chars())
        .filter(|ch| ch.0 == ch.1)
        .map(|ch| ch.0)
        .collect()
}

fn find_common_id() -> Option<String> {
    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);
    let mut box_ids: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    box_ids.sort();

    for i in 0..box_ids.len() {
        let mut diff = 0;
        if i != box_ids.len() - 1 {
            for (a, b) in box_ids[i].chars().zip(box_ids[i+1].chars()) {
                if a != b {
                    diff += 1;
                }
            }
            if diff == 1 {
                return Some(get_common_chars(&box_ids[i], &box_ids[i+1]));
            }
        }
    }
    None
}

fn main() {
    println!("Common letters in the box ids: {}",
             match find_common_id() {
                 Some(s) => s,
                 None => "NA".to_string()
             });
}

