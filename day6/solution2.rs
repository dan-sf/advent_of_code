use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
#[derive(Hash)]
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point { }

fn get_max_coords(coords: &Vec<Point>) -> (i32, i32) {
    let mut max_x = coords.iter().map(|p| p.x).max().unwrap();
    let mut max_y = coords.iter().map(|p| p.y).max().unwrap();
    max_x += 1;
    max_y += 1;
    (max_x, max_y)
}

fn get_safe_region_area(max_x: i32, max_y: i32, coords: &Vec<Point>) -> i32 {
    let mut safe_region = 0;
    for row in 0..max_x {
        for col in 0..max_y {
            let dist: i32 = coords.iter()
                .map(|p| get_manhattan_dist(&Point { x: col, y: row }, p)).sum();
            if dist < 10000 {
                safe_region += 1;
            }
        }
    }
    safe_region
}

fn get_manhattan_dist(start: &Point, end: &Point) -> i32 {
    let output = i32::abs(start.x - end.x) + i32::abs(start.y - end.y);
    output
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
    let (max_x, max_y) = get_max_coords(&coords);
    let safe_region = get_safe_region_area(max_x, max_y, &coords);
    println!("Size of the safe region: {}", safe_region);
}

