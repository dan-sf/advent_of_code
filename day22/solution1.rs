use std::fs;
use std::io;
use std::io::BufRead;


fn parse_input(path: &str) -> (usize, (usize, usize)) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);
    let mut depth: usize = 0;
    let mut target: (usize, usize) = (0, 0);

    for line in reader.lines() {
        let dim: Vec<String> = line.unwrap().replace(" ", "")
            .split(':').map(|r| r.to_string()).collect();
        if dim[0] == String::from("depth") {
            depth = dim[1].parse().unwrap();
        }
        if dim[0] == String::from("target") {
            target = (
                dim[1].split(',').collect::<Vec<&str>>()[0].parse().unwrap(),
                dim[1].split(',').collect::<Vec<&str>>()[1].parse().unwrap(),
            );
        }
    }

    (depth, target)
}

fn generate_cave(depth: usize, target: (usize, usize)) -> Vec<Vec<u64>> {
    let mut cave: Vec<Vec<u64>> = vec![vec![0;target.0+1];target.1+1];

    // Get geologic index
    for y in 0..cave.len() {
        for x in 0..cave[0].len() {
            if (x, y) == target || (x, y) == (0, 0) {
                continue;
            }

            if y == 0 {
                cave[y][x] = x as u64 * 16807;
            } else if x == 0 {
                cave[y][x] = y as u64 * 48271;
            } else {
                cave[y][x] = cave[y-1][x] * cave[y][x-1];
                println!("{}", cave[y][x]);
            }
        }
    }

    // Get erosion level
    for y in 0..cave.len() {
        for x in 0..cave[0].len() {
            cave[y][x] = ((cave[y][x] + depth as u64) % 20183) % 3;
        }
    }

    cave
}

fn main() {
    let (depth, target) = parse_input("input.txt");
    println!("{}, {:?}", depth, target);
    println!("cave: {:?}", generate_cave(depth, target));
}

