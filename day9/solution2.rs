use std::fs;
use std::io::Read;

fn play_game(players: i32, last_marble: i32) -> i32 {
    let mut player_scores: Vec<i32> = vec![0;players as usize];
    let mut player_index: usize = 3;

    let mut circle: Vec<i32> = vec![0, 2, 1];
    let mut current: i32 = 1;

    for marble in 3..(last_marble+1) {
        if marble % 23 == 0 {
            let mut remove_index = current - 7;
            if remove_index < 0 {
                remove_index += circle.len() as i32;
            }
            player_scores[player_index] += marble + circle.remove(remove_index as usize);
            current = remove_index;
        } else {
            current += 2;
            if current as usize > circle.len() {
                if current as usize == circle.len() {
                    current = 0;
                } else {
                    current = 1;
                }
            }
            circle.insert(current as usize, marble);
        }

        player_index += 1;
        player_index %= player_scores.len();
    }
    *player_scores.iter().max().unwrap()
}

fn main() {
    let mut input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");

    let mut read_string = String::new();
    input.read_to_string(&mut read_string).unwrap();
    let split_input = read_string.as_str().split(' ').collect::<Vec<&str>>();
    let players = split_input[0].parse::<i32>().unwrap();
    let last_marble = split_input[6].parse::<i32>().unwrap();

    for i in 10..300 {
        println!("Winning score: {} {}", play_game(players, i), i);
    }
}

