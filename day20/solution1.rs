use std::fs;
use std::io::Read;
use std::slice::Iter;


#[derive(Debug, Clone, Copy)]
enum Location {
    Room,
    Door,
    Wall,
}

fn parse_input(path: &str) -> Vec<char> {
    let mut input = fs::File::open(path)
        .expect("Something went wrong reading the file");

    let mut regex = String::new();
    input.read_to_string(&mut regex).unwrap();

    let mut regex_chars: Vec<char> = regex.trim_end().chars().collect();
    regex_chars
}

fn navigate_map(map: &mut Vec<Vec<Location>>, mut or_stack: Vec<char>, mut regex_iter: Iter<char>) {
    let mut next_ch: char = 'X';
    if let Some(ch) = regex_iter.next() {
        next_ch = *ch;
    } else {
        return;
    }
    println!("testing: {}", next_ch);
}

fn max_doors(map: Vec<Vec<Location>>) -> i32 {
    0
}

fn main() {
    let mut map = vec![vec![Location::Wall;50];50];
    let mut or_stack: Vec<char> = vec![];
    let mut regex_chars = parse_input("input.txt");
    let mut regex_iter = regex_chars.iter();

    navigate_map(&mut map, or_stack, regex_iter);
    println!("Largest number of doors: {:?}", max_doors(map));
}

