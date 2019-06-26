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

        // Get xy wh from the string
        let parts: Vec<&str> = unwrap_line.split(['@', ',', ':', 'x'].as_ref()).collect();
        let (x, y, width, height) = (
            parts[1].trim().parse::<i32>().unwrap(),
            parts[2].trim().parse::<i32>().unwrap(),
            parts[3].trim().parse::<i32>().unwrap(),
            parts[4].trim().parse::<i32>().unwrap()
        );

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

