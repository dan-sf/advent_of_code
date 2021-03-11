use std::fs;
use std::io::Read;
use std::slice::Iter;
use std::collections::HashSet;


#[derive(Debug, Clone, Copy)]
enum Location {
    Room,
    DoorNS,
    DoorEW,
    Wall,
    Start,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

// Read regex to a string and return a list of chars
fn parse_input(path: &str) -> Vec<char> {
    let mut input = fs::File::open(path)
        .expect("Something went wrong reading the file");

    let mut regex = String::new();
    input.read_to_string(&mut regex).unwrap();

    let regex_chars: Vec<char> = regex.trim_end().chars().collect();
    regex_chars
}

// Recursively walk the map while iterating over the regex
fn navigate_map(map: &mut Vec<Vec<Location>>, mut pos: Position, regex_iter: &mut Iter<char>) -> char {
    let mut output_ch = ' ';

    while let Some(ch) = regex_iter.next() {
        output_ch = *ch;
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
                output_ch = navigate_map(map, pos, regex_iter);

                // We need to continue to recurse if we get | back from the function call
                while output_ch == '|' {
                    output_ch = navigate_map(map, pos, regex_iter);
                }
            },
            ')' | '|' | '$' => {
                return output_ch;
            },
            '^' => { },
            _ => {
                panic!("Unexpected char in input regex");
            }
        };
    }

    output_ch
}

fn _debug_print(map: &Vec<Vec<Location>>) {
    println!();
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

// Trim the map down to only what is walkable. This function is only needed for debug printing the
// map
fn _trim_map(map: &mut Vec<Vec<Location>>) {
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

// Use DFS to walk the map keeping track of the room that is the most doors away from the start
fn max_doors(map: &Vec<Vec<Location>>, start: Position) -> i32 {

    let mut max_count = 0;
    let mut visited: HashSet<Position> = HashSet::new();
    fn traverse(map: &Vec<Vec<Location>>, pos: Position, count: i32, visited: &mut HashSet<Position>, max_count: &mut i32) {
        if pos.row >= map.len() || pos.col >= map[0].len() || visited.contains(&pos) {
            return;
        }

        if count > *max_count {
            *max_count = count;
        }

        visited.insert(pos);

        if let Location::Wall = map[pos.row + 1][pos.col] { }
        else {
            if let Location::Wall = map[pos.row + 2][pos.col] { }
            else {
                traverse(map, Position { row: pos.row + 2, col: pos.col }, count+1, visited, max_count);
            }
        }
        if let Location::Wall = map[pos.row - 1][pos.col] { }
        else {
            if let Location::Wall = map[pos.row - 2][pos.col] { }
            else {
                traverse(map, Position { row: pos.row - 2, col: pos.col }, count+1, visited, max_count);
            }
        }
        if let Location::Wall = map[pos.row][pos.col + 1] { }
        else {
            if let Location::Wall = map[pos.row][pos.col + 2] { }
            else {
                traverse(map, Position { row: pos.row, col: pos.col + 2 }, count+1, visited, max_count);
            }
        }
        if let Location::Wall = map[pos.row][pos.col - 1] { }
        else {
            if let Location::Wall = map[pos.row][pos.col - 2] { }
            else {
                traverse(map, Position { row: pos.row, col: pos.col - 2 }, count+1, visited, max_count);
            }
        }
    }

    traverse(&map, start, 0, &mut visited, &mut max_count);
    max_count
}

fn main() {
    let size: usize = 5000; // Arbitrary large enough initial map
    let mut map = vec![vec![Location::Wall;size];size];
    let regex_chars = parse_input("input.txt");
    let mut regex_iter = regex_chars.iter();
    let start = Position { row: size/2, col: size/2 };

    navigate_map(&mut map, start, &mut regex_iter);
    map[size/2][size/2] = Location::Start;
    let doors =  max_doors(&map, start);
    //_trim_map(&mut map);
    //_debug_print(&map);
    println!("Largest number of doors: {:?}", doors);
}

