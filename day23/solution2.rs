use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

#[derive(Debug)]
struct NanoBot {
    pos: (i32, i32, i32),
    radius: i32,
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

fn get_range(a: &NanoBot, b: &NanoBot) -> i32 {
    // i32::abs(a.pos.0 - b.pos.0) +
    // i32::abs(a.pos.1 - b.pos.1) +
    // i32::abs(a.pos.2 - b.pos.2)
    get_range_pos(a.pos, b.pos)
}

fn get_range_pos(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    i32::abs(a.0 - b.0) +
    i32::abs(a.1 - b.1) +
    i32::abs(a.2 - b.2)
}

fn sum_point(pos: (i32, i32, i32)) -> i32 {
    pos.0 + pos.1 + pos.2
}

fn get_bots_in_range(pos: (i32, i32, i32), nano_bots: &Vec<NanoBot>) -> i32 {
    let mut output = 0;
    for bot in nano_bots.iter() {
        if get_range_pos(pos, bot.pos) <= bot.radius {
            output += 1;
        }
    }
    output
}

fn point_search(start: (i32, i32, i32), mut bot_count: i32, nano_bots: &Vec<NanoBot>) -> (i32, i32, i32) {

    let mut output_point = start;
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut queue: Vec<(i32, (i32, i32, i32))> = Vec::new();
    queue.push((bot_count, start));

    while !queue.is_empty() {
        let (cur_bot_count, cur_pos) = queue.pop().unwrap();
        visited.insert(cur_pos);

        if cur_bot_count < bot_count {
            continue;
        }

        for step in &[(1, 0, 0), (0, 1, 0), (0, 0, 1),
                      (-1, 0, 0), (0, -1, 0), (0, 0, -1)] {
            let new_pos = (cur_pos.0 + step.0, cur_pos.1 + step.1, cur_pos.2 + step.2);
            let new_bot_count = get_bots_in_range(new_pos, nano_bots);

            if new_bot_count > bot_count {
                output_point = new_pos;
                bot_count = new_bot_count;
            } else if new_bot_count < bot_count {
                continue;
            } else if new_bot_count == bot_count && sum_point(new_pos) < sum_point(output_point) {
                output_point = new_pos;
            }

            queue.push((bot_count, new_pos));
        }
    }

    output_point
}

fn main() {
    let nano_bots = parse_input("input.txt");
    let mut max_bot = &nano_bots[0];
    let mut max_bots = 0;

    for i in 0..nano_bots.len() {
        let mut bots_near = 0;
        for j in 0..nano_bots.len() {
            let bot_dist = get_range(&nano_bots[i], &nano_bots[j]);
            if i != j && (bot_dist <= nano_bots[i].radius || bot_dist <= nano_bots[j].radius) {
                bots_near += 1;
            }
        }

        if bots_near > max_bots {
            max_bots = bots_near;
            max_bot = &nano_bots[i];
        }
    }

    println!("max_bots: {}, max_bot: {:?}", max_bots, max_bot);

    let result = point_search(max_bot.pos, max_bots, &nano_bots);
    println!("output_point: {:?}", result);

    //let mut count = 1;
    //for bot in nano_bots.iter() {
    //    if bot.pos != strongest_bot.pos && strongest_bot.radius >= get_range(strongest_bot, bot) {
    //        count += 1;
    //    }
    //}

    //println!("Nanobots in range: {:?}", count);
}

