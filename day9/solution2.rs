use std::fs;
use std::io::Read;
use std::collections::VecDeque;

// To solve this day I needed to update to using a VecDeque which simplifies the code quite a bit
// (didn't think of it for problem1), it also more closely models the problem. And I had to use i64
// ints over i32

fn play_game(players: i64, last_marble: i64) -> i64 {
    let mut player_scores: Vec<i64> = vec![0;players as usize];
    let mut player_index: usize = 3;

    let mut circle: VecDeque<i64> = VecDeque::new();
    circle.push_back(0); circle.push_back(2); circle.push_back(1);

    for marble in 3..(last_marble+1) {
        if marble % 23 == 0 {
            circle.rotate_right(7);
            player_scores[player_index] += marble + circle.pop_back().unwrap();
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(marble);
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
    let players = split_input[0].parse::<i64>().unwrap();
    let last_marble = split_input[6].parse::<i64>().unwrap();
    println!("Winning score: {} ", play_game(players, last_marble*100));
}

