use std::fs;
use std::io;
use std::io::BufRead;


#[derive(Copy, Clone)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}

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

fn generate_cave(depth: usize, target: (usize, usize)) -> Vec<Vec<Region>> {
    let mut value_cave: Vec<Vec<u64>> = vec![vec![0;target.0*2];target.1*2];
    let mut output_cave: Vec<Vec<Region>> = vec![vec![Region::Rocky;target.0*2];target.1*2];

    // Get geologic index/erosion level
    for y in 0..value_cave.len() {
        for x in 0..value_cave[0].len() {
            if (x, y) == target || (x, y) == (0, 0) {
                value_cave[y][x] = (depth as u64 % 20183) % 3;
                continue;
            }

            if y == 0 {
                value_cave[y][x] = ((x as u64 * 16807) + depth as u64) % 20183;
            } else if x == 0 {
                value_cave[y][x] = ((y as u64 * 48271) + depth as u64) % 20183;
            } else {
                value_cave[y][x] = ((value_cave[y-1][x] * value_cave[y][x-1]) + depth as u64) % 20183;
            }
        }
    }

    // Get region type values
    for y in 0..value_cave.len() {
        for x in 0..value_cave[0].len() {
            match value_cave[y][x] % 3 {
                1 => { output_cave[y][x] = Region::Wet; },
                2 => { output_cave[y][x] = Region::Narrow; },
                _ => { },
            }
        }
    }

    output_cave
}

fn main() {
    let (depth, target) = parse_input("input.txt");
    let cave = generate_cave(depth, target);
}

