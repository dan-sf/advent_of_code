use std::fs;
use std::io;
use std::io::BufRead;

// Parse x, y from the input string
fn get_xy(line: &String) -> (i32, i32) {
    let space_split = line.split(' ').collect::<Vec<&str>>()[2];
    let mut string_xy = space_split.to_string();
    string_xy.pop(); // Remove last char
    let xy = string_xy.as_str().split(',').collect::<Vec<&str>>();
    let x = xy[0].parse::<i32>().unwrap();
    let y = xy[1].parse::<i32>().unwrap();
    (x, y)
}

// Parse width, height from the input string
fn get_wh(line: &String) -> (i32, i32) {
    let string_wh = line.split(' ').collect::<Vec<&str>>()[3];
    let wh = string_wh.split('x').collect::<Vec<&str>>();
    let width = wh[0].parse::<i32>().unwrap();
    let height = wh[1].parse::<i32>().unwrap();
    (width, height)
}

fn main() {
    let mut fabric = vec![vec![0; 1000]; 1000];

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    for line in reader.lines() {
        let unwrap_line = line.unwrap();

        // Get xy from the string
        let (x, y) = get_xy(&unwrap_line);

        // Get width height from the string
        let (width, height) = get_wh(&unwrap_line);

        for i in 0..height {
            for j in 0..width {
                let row = (i + y) as usize;
                let col = (j + x) as usize;
                fabric[row][col] += 1;
            }
        }
    }

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    for line in reader.lines() {
        let unwrap_line = line.unwrap();

        // Get xy from the string
        let (x, y) = get_xy(&unwrap_line);

        // Get width height from the string
        let (width, height) = get_wh(&unwrap_line);

        let mut square = vec![0; (height*width) as usize];
        let mut index = 0;
        for i in 0..height {
            for j in 0..width {
                square[index] = fabric[(i+y) as usize][(j+x) as usize];
                index += 1;
            }
        }

        if square.len() == square.iter().fold(0, |a,&b| {a+(b as usize)}) {
            println!("Non-overlapping claim: {}", unwrap_line);
            break;
        }
    }
}

