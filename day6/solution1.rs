use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

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

fn construct_grid(coords: &Vec<Point>) -> Vec<Vec<Option<Point>>> {
    let mut grid: Vec<Vec<Option<Point>>> = Vec::new();
    let mut max_x = coords.iter().map(|p| p.x).max().unwrap();
    let mut max_y = coords.iter().map(|p| p.y).max().unwrap();

    // We need to add 1 to the max x and y to account for the origin
    max_x += 1;
    max_y += 1;
    for _ in 0..max_y {
        grid.push(vec![None;max_x as usize]);
    }

    grid
}

fn get_manhattan_dist(start: &Point, end: &Point) -> i32 {
    let output = i32::abs(start.x - end.x) + i32::abs(start.y - end.y);
    output
}

fn fill_grid(grid: &mut Vec<Vec<Option<Point>>>, coords: &Vec<Point>) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let cur_point = Point { x: col as i32, y: row as i32 };
            let mut min_dist: i32 = (grid.len() + grid[0].len()) as i32;
            let mut min_point = Point { x: 0, y: 0 };
            let mut mins = Vec::new();
            for p in coords.iter() {
                let dist = get_manhattan_dist(&cur_point, &p);
                if dist <= min_dist {
                    min_dist = dist;
                    min_point.x = p.x;
                    min_point.y = p.y;
                    mins.push(min_dist)
                }
            }
            let mut cmins = HashMap::new();
            for i in mins.iter() {
                let c = cmins.entry(i).or_insert(0);
                *c += 1;
            }
            if cmins[&min_dist] <= 1 {
                grid[row][col] = Some(min_point);
            }
        }
    }
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

fn get_largest_area(grid: Vec<Vec<Option<Point>>>) -> i32 {
    let mut infinite_set: HashSet<Point> = HashSet::new();
    for i in 0..grid[0].len() {
        if let Some(p) = &grid[0][i] {
            infinite_set.insert(*p);
        }
        if let Some(p) = &grid[grid.len()-1][i] {
            infinite_set.insert(*p);
        }
    }
    for j in 0..grid.len() {
        if let Some(p) = &grid[j][0] {
            infinite_set.insert(*p);
        }
        if let Some(p) = &grid[j][grid[0].len()-1] {
            infinite_set.insert(*p);
        }
    }

    let mut count = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if let Some(p) = grid[row][col] {
                if !infinite_set.contains(&p) {
                    let c = count.entry(p).or_insert(0);
                    *c += 1;
                }
            }
        }
    }

    *count.values().max().unwrap()
}

fn main() {
    let coords = load_coordinates();
    let mut grid = construct_grid(&coords);
    fill_grid(&mut grid, &coords);
    let largest_area = get_largest_area(grid);
    println!("Size of the largest area: {}", largest_area);
}

