use std::fs;
use std::io;
use std::io::BufRead;


fn main() {
    let mut fabric = vec![vec![0; 1000]; 1000];

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    for line in reader.lines() {
        let unwrap_line = line.unwrap();

        // Get xy from the string
        let mut space_split = unwrap_line.split(' ').collect::<Vec<&str>>()[2];
        let mut string_xy = space_split.to_string();
        string_xy.pop(); // Remove last char
        let xy = string_xy.as_str().split(',').collect::<Vec<&str>>();
        let x = xy[0].parse::<i32>().unwrap();
        let y = xy[1].parse::<i32>().unwrap();

        // Get width height from the string
        let string_wh = unwrap_line.split(' ').collect::<Vec<&str>>()[3];
        let wh = string_wh.split('x').collect::<Vec<&str>>();
        let width = wh[0].parse::<i32>().unwrap();
        let height = wh[1].parse::<i32>().unwrap();

        for i in 0..height {
            for j in 0..width {
                let row = (i + y) as usize;
                let col = (j + x) as usize;
                fabric[row][col] += 1;
            }
        }
    }

    let mut total_inches = 0;
    for row in fabric.iter() {
        for col in row.iter() {
            if col > &1 {
                total_inches += 1;
            }
        }
    }

    println!("Total square inches: {}", total_inches);
}

