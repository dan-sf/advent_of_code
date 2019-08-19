use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn construct_grid(coords: &Vec<Point>) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for p in coords.iter() {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }

    // We need to add 1 to the max x and y to account for the origin
    max_x += 1;
    max_y += 1;
    for _i in 0..max_y {
        grid.push(vec![0;max_x as usize]);
    }

    grid
}

fn load_coordinates() -> Vec<Point> {
    let mut output = Vec::new();

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    for line in reader.lines() {
        let line = line.unwrap().replace(" ", "");
        let coord_vec: Vec<&str> = line.split(',').collect();
        let x: i32 = coord_vec[0].parse().unwrap();
        let y: i32 = coord_vec[1].parse().unwrap();
        let p = Point { x: x, y: y };
        output.push(p);
    }
    output
}

fn main() {
    let coords = load_coordinates();
    let grid = construct_grid(&coords);
    println!("coords: {:?}, grid: {:?}", coords, grid[0]);
}


