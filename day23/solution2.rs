use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::BinaryHeap;


#[derive(Debug)]
struct NanoBot {
    pos: (i32, i32, i32),
    radius: i32,
}

// We define a cube using a single corner (smallest corner point) and an edge length
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
    edge: i32,
}

impl Cube {
    fn get_pos(self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    fn get_corners(self) -> [(i32, i32, i32); 8] {
        let cube_corners = [
            (self.x, self.y, self.z),
            (self.x, self.y, self.z + self.edge),
            (self.x, self.y + self.edge, self.z),
            (self.x, self.y + self.edge, self.z + self.edge),
            (self.x + self.edge, self.y, self.z),
            (self.x + self.edge, self.y, self.z + self.edge),
            (self.x + self.edge, self.y + self.edge, self.z),
            (self.x + self.edge, self.y + self.edge, self.z + self.edge)
        ];
        cube_corners
    }

    fn new_pos(pos: (i32, i32, i32), edge: i32) -> Cube {
        Cube { x: pos.0, y: pos.1, z: pos.2, edge: edge }
    }
}

fn parse_input(path: &str) -> Vec<NanoBot> {
    let input = File::open(path)
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(input);

    let mut output: Vec<NanoBot> = vec![];
    for line in reader.lines() {
        let split = line.unwrap().split(">, ").map(|r| r.to_string()).collect::<Vec<String>>();
        let radius = split[1][2..].parse::<i32>().unwrap();
        let pos_list = split[0][5..].split(",").map(|r| r.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        output.push(NanoBot { pos: (pos_list[0], pos_list[1], pos_list[2]), radius: radius });
    }
    output
}

fn get_manhattan_dist(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    i32::abs(a.0 - b.0) +
    i32::abs(a.1 - b.1) +
    i32::abs(a.2 - b.2)
}

fn is_bot_in_cube(cube: &Cube, bot: &NanoBot) -> bool {
    (bot.pos.0 >= cube.x && bot.pos.0 <= cube.x + cube.edge) &&
    (bot.pos.1 >= cube.y && bot.pos.1 <= cube.y + cube.edge) &&
    (bot.pos.2 >= cube.z && bot.pos.2 <= cube.z + cube.edge)
}

// Return the range, min, and max x/y/z values from the nano bot list
fn get_bot_range(nano_bots: &Vec<NanoBot>) -> ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)) {
    let (mut min_x, mut min_y, mut min_z) = (std::i32::MAX, std::i32::MAX, std::i32::MAX);
    let (mut max_x, mut max_y, mut max_z) = (std::i32::MIN, std::i32::MIN, std::i32::MIN);
    for bot in nano_bots.iter() {
        min_x = i32::min(bot.pos.0 - bot.radius, min_x);
        min_y = i32::min(bot.pos.1 - bot.radius, min_y);
        min_z = i32::min(bot.pos.2 - bot.radius, min_z);
        max_x = i32::max(bot.pos.0 + bot.radius, max_x);
        max_y = i32::max(bot.pos.1 + bot.radius, max_y);
        max_z = i32::max(bot.pos.2 + bot.radius, max_z);
    }

    ((max_x-min_x, max_y-min_y, max_z-min_z), (min_x, min_y, min_z), (max_x, max_y, max_z))
}

fn max_point(pos: (i32, i32, i32)) -> i32 {
    i32::max(i32::max(pos.0, pos.1), pos.2)
}

fn sum_point(pos: (i32, i32, i32)) -> i32 {
    pos.0 + pos.1 + pos.2
}

// Perform an ordered BFS-like search on the space. We create a large cube that holds all the bots
// then continually divide the cube until we are at the smallest possible cube with the highest bot
// count
fn get_dist_to_best_point(nano_bots: &Vec<NanoBot>) -> i32 {
    let (range, mins, _) = get_bot_range(nano_bots);
    let edge = get_smallest_edge(max_point(range));

    let mut queue: BinaryHeap<(i32, Cube)> = BinaryHeap::new();
    queue.push((nano_bots.len() as i32, Cube::new_pos(mins, edge))); // Use the bot count to order the heap

    while !queue.is_empty() {
        let (_bots, cube) = queue.pop().unwrap();

        if cube.edge == 0 {
            let max_bots: i32 = nano_bots.iter().map(
                |b| if in_range(b, cube.get_pos()) { 1 } else { 0 }).sum();
            println!("Best point: {:?}", cube.get_pos());
            println!("Bots in range: {}", max_bots);
            let min_dist = sum_point(cube.get_pos());
            return min_dist;
        }

        let new_edge = cube.edge / 2;
        let base_cube = Cube::new_pos(cube.get_pos(), new_edge);
        for corner in base_cube.get_corners().iter() {
            let cube = Cube::new_pos(*corner, new_edge);
            let bots = get_bots_in_cube(&cube, nano_bots);
            queue.push((bots, cube));
        }
    }

    unreachable!()
}

// Get the smallest power of 2 that is larger than the given value
fn get_smallest_edge(value: i32) -> i32 {
    let mut start = 31;
    loop {
        if 1<<start & value != 0 {
            break;
        }
        start -= 1;
    }
    1<<(start + 1)
}

// This function isn't fully correct, we just count bots for a given cube if the bots are within
// the cube, or the bot is within range of any of the cube's corners. I couldn't get a fully
// correct version implemented but this worked for my input...
fn get_bots_in_cube(cube: &Cube, nano_bots: &Vec<NanoBot>) -> i32 {
    let cube_corners = cube.get_corners();
    let mut output = 0;
    for bot in nano_bots.iter() {
        // Check if a bot is within a cube
        if is_bot_in_cube(cube, bot) {
            output += 1;
            continue;
        }

        // Check if a bot is within range of any of the corners
        for pos in cube_corners.iter() {
            if in_range(bot, *pos) {
                output += 1;
                break;
            }
        }
    }
    output
}

fn in_range(bot: &NanoBot, pos: (i32, i32, i32)) -> bool {
    get_manhattan_dist(bot.pos, pos) <= bot.radius
}

fn main() {
    let nano_bots = parse_input("input.txt");
    println!("Shortest manhattan distance: {:?}", get_dist_to_best_point(&nano_bots));
}

