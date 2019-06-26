use std::fs;
use std::io;
use std::io::BufRead;

// Parse x, y, width, height from the input string
fn get_xy_wh(line: &String) -> (i32, i32, i32, i32) {
    let parts: Vec<&str> = line.split(['@', ',', ':', 'x'].as_ref()).collect();
    let (x, y, width, height) = (
        parts[1].trim().parse::<i32>().unwrap(),
        parts[2].trim().parse::<i32>().unwrap(),
        parts[3].trim().parse::<i32>().unwrap(),
        parts[4].trim().parse::<i32>().unwrap()
    );
    (x, y, width, height)
}

fn main() {
    let mut fabric = vec![vec![0; 1000]; 1000];

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    for line in reader.lines() {
        let unwrap_line = line.unwrap();

        // Get xy, wh from the string
        let (x, y, width, height) = get_xy_wh(&unwrap_line);

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

        // Get xy wh from the string
        let (x, y, width, height) = get_xy_wh(&unwrap_line);

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

