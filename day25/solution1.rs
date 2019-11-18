use std::io::{BufReader, BufRead};
use std::fs::File;


fn parse_input(path: &str) -> Vec<Vec<i32>> {
    let input = File::open(path)
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(input);

    let mut output: Vec<Vec<i32>> = vec![];
    for line in reader.lines() {
        output.push(
            line.unwrap().split(",").map(|r| r.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        );
    }

    output
}

// Check if two points are within the given Manhattan distance
fn is_in_range(start: &Vec<i32>, end: &Vec<i32>, dist: i32) -> bool {
    let mdist: i32 = start.iter().zip(end.iter()).map(|p| i32::abs(p.0 - p.1)).sum();
    if mdist <= dist {
        return true;
    }
    false
}

// Return the number of constellations in the points
fn get_constellations(mut points: Vec<Vec<i32>>) -> i32 {
    let mut constellations = 0;

    loop {
        let mut constellation_points: Vec<Vec<i32>> = vec![points.pop().unwrap()];

        loop {
            let mut removal: Vec<usize> = vec![];
            let mut added = false;
            for (i, point) in points.iter().enumerate() {
                let mut pushed = false;
                for cp in constellation_points.iter() {
                    if is_in_range(&cp, &point, 3) {
                        removal.push(i);
                        pushed = true;
                        added = true;
                        break;
                    }
                }
                if pushed {
                    constellation_points.push(point.iter().map(|r| *r).collect::<Vec<i32>>());
                }
            }

            removal.reverse();
            for r in removal.iter() {
                points.remove(*r);
            }

            if !added {
                break;
            }
        }

        constellations += 1;
        if points.is_empty() {
            break;
        }
    }

    return constellations;
}

fn main() {
    let points = parse_input("input.txt");
    let constellations = get_constellations(points);
    println!("Constellations: {}", constellations);
}

