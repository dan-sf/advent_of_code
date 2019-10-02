use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn get_points(point_a: &str, point_b: &str) -> Vec<Point> {
    let (a_x_y, a_val) = point_a.split('=').collect();
    let (b_x_y, b_val) = point_b.split('=').collect();
    if a_x_y == 'x' {
        if a_val.contains('.') {
            let (start, end) = a_val.split("..").map(|r| r.parse::<usize>()).collect();

struct Point {
    x: usize,
    y: usize,
}

fn parse_input(path: &str) -> (Vec<RegChange>, Vec<Vec<i32>>) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut points: Vec<(usize, usize)> = Vec::new();

    for line in reader.lines() {
        line = line.unwrap();
        point = line.replace(" ", "").split(',');
        points.extend(get_points(point[0], point[1]));
    }

}

// x=452, y=1077..1087
// y=782, x=505..509

    // let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
    // let mut lines_iter = lines.iter();

    // // Parse the first part of the input
    // while let Some(line) = lines_iter.next() {
    //     if line.is_empty() {
    //         break;
    //     }

    //     let before = &line.chars().collect::<Vec<char>>()[9..line.len()-1];
    //     let before = before.iter().filter(|c| !c.is_whitespace() && !(c == &&',')).collect::<Vec<&char>>();
    //     let before = before.iter().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

    //     let instruction = lines_iter.next().unwrap().split(' ').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    //     let after = &lines_iter.next().unwrap().chars().collect::<Vec<char>>()[9..line.len()-1];
    //     let after = after.iter().filter(|c| !c.is_whitespace() && !(c == &&',')).collect::<Vec<&char>>();
    //     let after = after.iter().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

    //     let _empty = lines_iter.next().unwrap();

    //     reg_events.push(
    //         RegChange {
    //             before: before,
    //             instruction: instruction,
    //             after: after,
    //         });
    // }

    // // Parse the second part of the input
    // while let Some(line) = lines_iter.next() {
    //     if line.is_empty() {
    //         continue;
    //     }
    //     let instruction = line.split(' ').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    //     example_program.push(instruction);
    // }

    // //println!("reg_events: {:?}, example_program: {}", reg_events.len(), example_program.len());
    // (reg_events, example_program)

fn main() {
    let points = parse_input("input.txt");
    //println!("points: {:?}", points);
}


