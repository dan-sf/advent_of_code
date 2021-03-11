use std::fs;
use std::io;
use std::io::BufRead;

// Basically the same code as solution1 we just print out the steps needed to simulate the lights
// at the end

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Light {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn load_light_data() -> Vec<Light> {
    let mut output = Vec::new();

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    // Parse out the position and velocity
    for line in reader.lines() {
        let line = line.unwrap()
            .replace("position=<", "")
            .replace("> velocity=<", ",")
            .replace(">", "")
            .replace(" ", "");
        let light_vec: Vec<&str> = line.split(',').collect();
        let p: (i32, i32) = (light_vec[0].parse().unwrap(), light_vec[1].parse().unwrap());
        let v: (i32, i32) = (light_vec[2].parse().unwrap(), light_vec[3].parse().unwrap());
        output.push(Light { position: p, velocity: v });
    }
    output
}

fn simulate(lights: &mut Vec<Light>) {
    for l in lights.iter_mut() {
        l.position.0 += l.velocity.0;
        l.position.1 += l.velocity.1;
    }
}

fn output_message(lights: &Vec<Light>, max_x: i32, min_x: i32, max_y: i32, min_y: i32) {
    let rows = max_y - min_y + 1;
    let columns = max_x - min_x + 1;

    let mut sky = vec![vec![0;columns as usize];rows as usize];

    // Fill sky grid
    for l in lights.iter() {
        let row = l.position.1 - min_y;
        let col = l.position.0 - min_x;
        sky[row as usize][col as usize] = 1;
    }

    // Print out the message
    for row in 0..rows {
        for col in 0..columns {
            if sky[row as usize][col as usize] == 1 {
                print!("#");
            } else {
                print!("-");
            }
        }
        println!("");
    }
}

fn get_max_mins(lights: &Vec<Light>) -> (i32, i32, i32, i32) {
    let (mut max_x, mut min_x, mut max_y, mut min_y) = (
        std::i32::MIN,
        std::i32::MAX,
        std::i32::MIN,
        std::i32::MAX
    );

    for l in lights.iter() {
        if l.position.0 > max_x { max_x = l.position.0; }
        if l.position.0 < min_x { min_x = l.position.0; }
        if l.position.1 > max_y { max_y = l.position.1; }
        if l.position.1 < min_y { min_y = l.position.1; }
    }
    (max_x, min_x, max_y, min_y)
}

fn main() {
    let mut lights = load_light_data();

    let (mut max_x, mut min_x, mut max_y, mut min_y) = get_max_mins(&lights);

    let mut steps = 0;
    while (max_y - min_y) > 10 {
        simulate(&mut lights);
        // Kind of lame we need to create new vars here to pull the max/mins out of the tuple
        let (_max_x, _min_x, _max_y, _min_y) = get_max_mins(&lights);
        max_x = _max_x; min_x = _min_x; max_y = _max_y; min_y = _min_y;
        steps += 1;
    }

    output_message(&lights, max_x, min_x, max_y, min_y);
    println!("Steps needed: {}", steps);
}

