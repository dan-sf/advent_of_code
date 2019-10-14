use std::fs;
use std::io;
use std::io::BufRead;


#[derive(Debug, Clone, Copy)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

fn parse_input(path: &str, size: usize) -> Vec<Vec<Acre>> {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut landscape: Vec<Vec<Acre>> = vec![vec![Acre::Open;size];size];

    for (row, line) in reader.lines().enumerate() {
        for (col, acre) in line.unwrap().chars().enumerate() {
            match acre {
                '.' => { },
                '|' => {
                    landscape[row][col] = Acre::Trees;
                },
                '#' => {
                    landscape[row][col] = Acre::Lumberyard;
                },
                _ => { },
            };
        }
    }
    landscape
}

fn _debug_print(landscape: &Vec<Vec<Acre>>) {
    for row in landscape.iter() {
        for col in row.iter() {
            match col {
                Acre::Open => { print!("."); },
                Acre::Lumberyard => { print!("#"); },
                Acre::Trees => { print!("|"); },
            }
        }
        println!("");
    }
    println!("");
}

fn open_increase(landscape: &Vec<Vec<Acre>>, row: usize, col: usize) -> Acre {
    let locations: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                      (0,  -1),          (0, 1),
                                      (1,  -1), (1, 0),  (1, 1),];

    let mut trees = 0;
    for loc in locations.iter() {
        let check_row = row as i32 + loc.0;
        let check_col = col as i32 + loc.1;
        if check_row >= 0 && check_row < landscape.len() as i32 && check_col >= 0 && check_col < landscape[0].len() as i32 {
            if let Acre::Trees = landscape[check_row as usize][check_col as usize] {
                trees += 1;
            }
        }
    }

    if trees >= 3 {
        return Acre::Trees;
    }
    Acre::Open
}

fn tree_increase(landscape: &Vec<Vec<Acre>>, row: usize, col: usize) -> Acre {
    let locations: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                      (0,  -1),          (0, 1),
                                      (1,  -1), (1, 0),  (1, 1),];

    let mut lumberyards = 0;
    for loc in locations.iter() {
        let check_row = row as i32 + loc.0;
        let check_col = col as i32 + loc.1;
        if check_row >= 0 && check_row < landscape.len() as i32 && check_col >= 0 && check_col < landscape[0].len() as i32 {
            if let Acre::Lumberyard = landscape[check_row as usize][check_col as usize] {
                lumberyards += 1;
            }
        }
    }

    if lumberyards >= 3 {
        return Acre::Lumberyard;
    }
    Acre::Trees
}

fn lumber_increase(landscape: &Vec<Vec<Acre>>, row: usize, col: usize) -> Acre {
    let locations: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                      (0,  -1),          (0, 1),
                                      (1,  -1), (1, 0),  (1, 1),];

    let mut trees = 0;
    let mut lumberyards = 0;
    for loc in locations.iter() {
        let check_row = row as i32 + loc.0;
        let check_col = col as i32 + loc.1;
        if check_row >= 0 && check_row < landscape.len() as i32 && check_col >= 0 && check_col < landscape[0].len() as i32 {
            if let Acre::Lumberyard = landscape[check_row as usize][check_col as usize] {
                lumberyards += 1;
            } else if let Acre::Trees = landscape[check_row as usize][check_col as usize] {
                trees += 1;
            }
        }
    }

    if lumberyards >= 1 && trees >= 1 {
        return Acre::Lumberyard;
    }
    Acre::Open
}

fn increase_time(landscape: Vec<Vec<Acre>>) -> Vec<Vec<Acre>> {
    let size = landscape.len();
    let mut output_landscape: Vec<Vec<Acre>> = vec![vec![Acre::Open;size];size];

    for row in 0..size {
        for col in 0..size {
            match landscape[row][col] {
                Acre::Open => {
                    output_landscape[row][col] = open_increase(&landscape, row, col);
                },
                Acre::Lumberyard => {
                    output_landscape[row][col] = lumber_increase(&landscape, row, col);
                },
                Acre::Trees => {
                    output_landscape[row][col] = tree_increase(&landscape, row, col);
                },
            };
        }
    }

    output_landscape
}

fn get_resource_value(landscape: Vec<Vec<Acre>>) -> i32 {
    let mut wooded = 0;
    let mut lumber = 0;
    for row in 0..landscape.len() {
        for col in 0..landscape[row].len() {
            match landscape[row][col] {
                Acre::Lumberyard => {
                    lumber += 1;
                },
                Acre::Trees => {
                    wooded += 1;
                },
                Acre::Open => { },
            };
        }
    }

    wooded * lumber
}

fn main() {
    let size = 50;
    let mut landscape = parse_input("input.txt", size);
    for _ in 0..10 {
        landscape = increase_time(landscape);
    }
    println!("Resource value: {}", get_resource_value(landscape));
}

