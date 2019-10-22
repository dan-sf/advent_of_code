use std::fs;
use std::io::Read;
use std::slice::Iter;


#[derive(Debug, Clone, Copy)]
enum Location {
    Room,
    DoorNS,
    DoorEW,
    Wall,
    Start,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

fn parse_input(path: &str) -> Vec<char> {
    let mut input = fs::File::open(path)
        .expect("Something went wrong reading the file");

    let mut regex = String::new();
    input.read_to_string(&mut regex).unwrap();

    let mut regex_chars: Vec<char> = regex.trim_end().chars().collect();
    regex_chars
}

fn navigate_map(map: &mut Vec<Vec<Location>>, mut pos: Position, or_stack: &mut Vec<char>, regex_iter: &mut Iter<char>) {

    while let Some(ch) = regex_iter.next() {
        match ch {
            'N' => {
                pos.row -= 1;
                map[pos.row][pos.col] = Location::DoorNS;
                pos.row -= 1;
                map[pos.row][pos.col] = Location::Room;
            },
            'S' => {
                pos.row += 1;
                map[pos.row][pos.col] = Location::DoorNS;
                pos.row += 1;
                map[pos.row][pos.col] = Location::Room;
            },
            'E' => {
                pos.col += 1;
                map[pos.row][pos.col] = Location::DoorEW;
                pos.col += 1;
                map[pos.row][pos.col] = Location::Room;
            },
            'W' => {
                pos.col -= 1;
                map[pos.row][pos.col] = Location::DoorEW;
                pos.col -= 1;
                map[pos.row][pos.col] = Location::Room;
            },
            '(' => {
                //or_stack.push(*ch);
                navigate_map(map, pos, or_stack, regex_iter);
            },
            ')' => {
                //or_stack.pop();
                return;
            },
            '^' => { },
            '$' => {
                return;
            },
            '|' => {
                //navigate_map(map, pos, or_stack, regex_iter);
                return;
            },
            _ => {
                panic!("Unexpected char in input regex");
            }
        };
    }
}

fn _debug_print(map: &Vec<Vec<Location>>) {
    for row in map.iter() {
        for col in row.iter() {
            match col {
                Location::Room => { print!("."); },
                Location::DoorNS => { print!("-"); },
                Location::DoorEW => { print!("|"); },
                Location::Wall => { print!("#"); },
                Location::Start => { print!("X"); },
            };
        }
        println!();
    }
}

fn trim_map(map: &mut Vec<Vec<Location>>) {
    let mut trim_rows: Vec<usize> = vec![];
    let mut trim_cols: Vec<usize> = vec![];

    // Remove northern walls
    loop {
        let row = map.remove(0);
        let walls = row.iter().map(|r| if let Location::Wall = r { 1 } else { 0 })
            .fold(0, |a,b| a+b);
        if walls == row.len() {
            continue;
        }
        map.insert(0, row);
        break;
    }

    // Remove southern walls
    loop {
        let row = map.pop().unwrap();
        let walls = row.iter().map(|r| if let Location::Wall = r { 1 } else { 0 })
            .fold(0, |a,b| a+b);
        if walls == row.len() {
            continue;
        }
        map.push(row);
        break;
    }

    // Remove western walls
    loop {
        let walls = map.iter().map(|r| if let Location::Wall = r[0] { 1 } else { 0 })
            .fold(0, |a,b| a+b);
        if walls == map.len() {
            for row in map.iter_mut() {
                row.remove(0);
            }
        } else {
            break;
        }
    }

    // Remove eastern walls
    loop {
        let walls = map.iter().map(|r| if let Location::Wall = r[r.len()-1] { 1 } else { 0 })
            .fold(0, |a,b| a+b);
        if walls == map.len() {
            for row in map.iter_mut() {
                row.pop();
            }
        } else {
            break;
        }
    }

    // Add walls to the outer perimeter
    map.insert(0, vec![Location::Wall;map[0].len()]);
    map.push(vec![Location::Wall;map[0].len()]);
    for row in map.iter_mut() {
        row.insert(0, Location::Wall);
        row.push(Location::Wall);
    }
}

fn max_doors(map: Vec<Vec<Location>>) -> i32 {
    0
}

fn main() {
    let size: usize = 50;
    let mut map = vec![vec![Location::Wall;size];size];
    let mut or_stack: Vec<char> = vec![];
    //let mut regex_chars = parse_input("input.txt");
    //let mut regex_chars = parse_input("input.test.txt");
    let mut regex_chars = parse_input("input.test.txt2");
    //let mut regex_chars = parse_input("input.test.txt3");
    //let mut regex_chars = parse_input("input.test.txt4");
    let mut regex_iter = regex_chars.iter();
    let mut start = Position { row: size/2, col: size/2 };

    //_debug_print(&map);

    navigate_map(&mut map, start, &mut or_stack, &mut regex_iter);
    map[size/2][size/2] = Location::Start;
    trim_map(&mut map);
    _debug_print(&map);
    //println!("Largest number of doors: {:?}", max_doors(map));
}

