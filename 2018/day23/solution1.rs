use std::io::{BufReader, BufRead};
use std::fs::File;

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
    i32::abs(a.pos.0 - b.pos.0) +
    i32::abs(a.pos.1 - b.pos.1) +
    i32::abs(a.pos.2 - b.pos.2)
}

fn main() {
    let nano_bots = parse_input("input.txt");
    let mut strongest_bot = &nano_bots[0];

    for i in 1..nano_bots.len() {
        if nano_bots[i].radius > strongest_bot.radius {
            strongest_bot = &nano_bots[i];
        }
    }

    let mut count = 1;
    for bot in nano_bots.iter() {
        if bot.pos != strongest_bot.pos && strongest_bot.radius >= get_range(strongest_bot, bot) {
            count += 1;
        }
    }

    println!("Nanobots in range: {:?}", count);
}

