use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

// @Note: Since this program uses a lot of stack space we need to increase the stack size for the
// main thread. Otherwise we will overflow, this can be done with the following command:
// rustc -C 'link-args=-Wl,-stack_size,0x80000000' solution2.rs  // 2 GB stack size

#[derive(Debug, Copy, Clone)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Tool {
    ClimbingGear,
    Torch,
    Neither,
}

fn parse_input(path: &str) -> (usize, (usize, usize)) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);
    let mut depth: usize = 0;
    let mut target: (usize, usize) = (0, 0);

    for line in reader.lines() {
        let dim: Vec<String> = line.unwrap().replace(" ", "")
            .split(':').map(|r| r.to_string()).collect();
        if dim[0] == String::from("depth") {
            depth = dim[1].parse().unwrap();
        }
        if dim[0] == String::from("target") {
            target = (
                dim[1].split(',').collect::<Vec<&str>>()[0].parse().unwrap(),
                dim[1].split(',').collect::<Vec<&str>>()[1].parse().unwrap(),
            );
        }
    }

    (depth, target)
}

fn generate_cave(depth: usize, target: (usize, usize)) -> Vec<Vec<Region>> {
    let mut value_cave: Vec<Vec<u64>> = vec![vec![0;target.0*2];target.1*2];
    let mut output_cave: Vec<Vec<Region>> = vec![vec![Region::Rocky;target.0*2];target.1*2];

    // Get geologic index/erosion level
    for y in 0..value_cave.len() {
        for x in 0..value_cave[0].len() {
            if (x, y) == target || (x, y) == (0, 0) {
                value_cave[y][x] = (depth as u64 % 20183) % 3;
                continue;
            }

            if y == 0 {
                value_cave[y][x] = ((x as u64 * 16807) + depth as u64) % 20183;
            } else if x == 0 {
                value_cave[y][x] = ((y as u64 * 48271) + depth as u64) % 20183;
            } else {
                value_cave[y][x] = ((value_cave[y-1][x] * value_cave[y][x-1]) + depth as u64) % 20183;
            }
        }
    }

    // Get region type values
    for y in 0..value_cave.len() {
        for x in 0..value_cave[0].len() {
            match value_cave[y][x] % 3 {
                1 => { output_cave[y][x] = Region::Wet; },
                2 => { output_cave[y][x] = Region::Narrow; },
                _ => { },
            }
        }
    }

    output_cave
}

fn get_fastest_time(cave: Vec<Vec<Region>>, target: (usize, usize)) -> i32 {
    let mut output = std::i32::MAX; // Should be i32 max
    let mut visited: HashSet<((usize, usize), Tool)> = HashSet::new();
    let mut equiped = Tool::Torch;

    fn traverse(cave: &Vec<Vec<Region>>, target: &(usize, usize), visited: &mut HashSet<((usize, usize), Tool)>, pos: (usize, usize), equiped: Tool, time: i32, output: &mut i32) {
        if pos == *target {
            if time < *output {
                *output = time;
            }
            return;
        }

        visited.insert((pos, equiped));

        let mut pos_list = vec![(pos.0+1, pos.1), (pos.0, pos.1+1)];
        if pos.0 > 0 { pos_list.push((pos.0-1, pos.1)); }
        if pos.1 > 0 { pos_list.push((pos.0, pos.1-1)); }

        for new_pos in pos_list.iter() {
            if !visited.contains(&(*new_pos, equiped)) && new_pos.0 < cave[0].len() && new_pos.1 < cave.len() {
                match cave[new_pos.1][new_pos.0] {
                    Region::Rocky => {
                        if let Tool::Neither = equiped {
                            traverse(cave, target, visited, *new_pos, Tool::Torch, time+8, output);
                            traverse(cave, target, visited, *new_pos, Tool::ClimbingGear, time+8, output);
                        } else {
                            traverse(cave, target, visited, *new_pos, equiped, time+1, output);
                        }
                    },
                    Region::Wet => {
                        if let Tool::Torch = equiped {
                            traverse(cave, target, visited, *new_pos, Tool::ClimbingGear, time+8, output);
                            traverse(cave, target, visited, *new_pos, Tool::Neither, time+8, output);
                        } else {
                            traverse(cave, target, visited, *new_pos, equiped, time+1, output);
                        }
                    },
                    Region::Narrow => {
                        if let Tool::ClimbingGear = equiped {
                            traverse(cave, target, visited, *new_pos, Tool::Torch, time+8, output);
                            traverse(cave, target, visited, *new_pos, Tool::Neither, time+8, output);
                        } else {
                            traverse(cave, target, visited, *new_pos, equiped, time+1, output);
                        }
                    },
                }
            }
        }
    }

    traverse(&cave, &target, &mut visited, (0, 0), Tool::Torch, 0, &mut output);

    output
}

fn get_fastest_time_CALL(cave: Vec<Vec<Region>>, target: (usize, usize)) -> i32 {
    let mut output = std::i32::MAX;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    get_fastest_time_BSF(&cave, (0, 0), target, &mut visited, Tool::Torch, 0, &mut output);
    output
}

fn get_fastest_time_BSF(cave: &Vec<Vec<Region>>, start: (usize, usize), target: (usize, usize), visited: &mut HashSet<(usize, usize)>, equiped: Tool, mut time: i32, output: &mut i32) {
    let mut queue: Vec<((usize, usize), i32)> = vec![(start, time)];

    while !queue.is_empty() {
        let (pos, time) = queue.remove(0);
        visited.insert(pos);

        if pos == target {
            println!("pos: {:?}, target: {:?}, time: {}, output: {}", pos, target, time, *output);
            if time < *output {
                *output = time;
            }
            return;
        }

        //println!("{:?}", pos);
        let mut pos_list = vec![(pos.0+1, pos.1), (pos.0, pos.1+1)];
        if pos.0 > 0 { pos_list.push((pos.0-1, pos.1)); }
        if pos.1 > 0 { pos_list.push((pos.0, pos.1-1)); }

        for new_pos in pos_list.iter() {
            if !visited.contains(new_pos) && new_pos.0 < cave[0].len() && new_pos.1 < cave.len() {
                queue.push((*new_pos, time+1));
                //visited.insert(*new_pos);
                match cave[new_pos.1][new_pos.0] {
                    Region::Rocky => {
                        if let Tool::Neither = equiped {
                            //visited.remove(new_pos);
                            get_fastest_time_BSF(cave, *new_pos, target, visited, Tool::Torch, time+7, output);
                            //visited.remove(new_pos);
                            get_fastest_time_BSF(cave, *new_pos, target, visited, Tool::ClimbingGear, time+7, output);
                            //visited.insert(*new_pos);
                        }
                    },
                    Region::Wet => {
                        if let Tool::Torch = equiped {
                            //visited.remove(new_pos);
                            get_fastest_time_BSF(cave, *new_pos, target, visited, Tool::ClimbingGear, time+7, output);
                            //visited.remove(new_pos);
                            get_fastest_time_BSF(cave, *new_pos, target, visited, Tool::Neither, time+7, output);
                            //visited.insert(*new_pos);
                        }
                    },
                    Region::Narrow => {
                        if let Tool::ClimbingGear = equiped {
                            //visited.remove(new_pos);
                            get_fastest_time_BSF(cave, *new_pos, target, visited, Tool::Torch, time+7, output);
                            //visited.remove(new_pos);
                            get_fastest_time_BSF(cave, *new_pos, target, visited, Tool::Neither, time+7, output);
                            //visited.insert(*new_pos);
                        }
                    },
                }
            }
        }
    }
}

fn main() {
    //let (depth, target) = parse_input("input.txt");
    let (depth, target) = parse_input("input.test.txt");
    let cave = generate_cave(depth, target);
    println!("{:?}", cave);
    println!("{:?}, {}", cave.len(), cave[0].len());
    //let mut cave: Vec<Vec<Region>> = vec![vec![Region::Rocky;22];838];
    //println!("{:?}",target);
    let output = get_fastest_time_CALL(cave, target);
    println!("{:?}",output);
}

