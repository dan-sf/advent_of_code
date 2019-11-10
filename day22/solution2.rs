use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::BTreeSet;


#[derive(Debug, Copy, Clone)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    let mut value_cave: Vec<Vec<u64>> = vec![vec![0;target.0+50];target.1+50];
    let mut output_cave: Vec<Vec<Region>> = vec![vec![Region::Rocky;target.0+50];target.1+50];

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

// Use Dijkstra's algo to find the sortest path to the target
fn get_fastest_time(cave: &Vec<Vec<Region>>, start: (usize, usize), target: (usize, usize)) -> i32 {
    let mut output = 0;
    let mut visited: HashSet<((usize, usize), Tool)> = HashSet::new();
    // Here we use a btree to keep the queue sorted by time
    let mut queue: BTreeSet<(i32, (usize, usize), Tool)> = BTreeSet::new();
    queue.insert((0, start, Tool::Torch));

    while !queue.is_empty() {
        let (time, pos, tool) = queue.take(&queue.iter().next().unwrap().clone()).unwrap();

        if pos == target {
            output = time;
            break;
        }

        // We don't need to add any nodes if we already visited this pos/tool combo
        if !visited.insert((pos, tool)) {
            continue;
        }

        // Add nodes for changing the right equipment given the region type
        match cave[pos.1][pos.0] {
            Region::Rocky => {
                match tool {
                    Tool::ClimbingGear => queue.insert((time+7, pos, Tool::Torch)),
                    Tool::Torch => queue.insert((time+7, pos, Tool::ClimbingGear)),
                    _ => unreachable!(),
                };
            },
            Region::Wet => {
                match tool {
                    Tool::ClimbingGear => queue.insert((time+7, pos, Tool::Neither)),
                    Tool::Neither => queue.insert((time+7, pos, Tool::ClimbingGear)),
                    _ => unreachable!(),
                };
            },
            Region::Narrow => {
                match tool {
                    Tool::Torch => queue.insert((time+7, pos, Tool::Neither)),
                    Tool::Neither => queue.insert((time+7, pos, Tool::Torch)),
                    _ => unreachable!(),
                };
            },
        };

        let mut pos_list = vec![(pos.0+1, pos.1), (pos.0, pos.1+1)];
        if pos.0 > 0 { pos_list.push((pos.0-1, pos.1)); }
        if pos.1 > 0 { pos_list.push((pos.0, pos.1-1)); }

        // For all node neighbors add to the queue if we can safely move to a new region with the
        // current tool we have
        for new_pos in pos_list.iter() {
            if new_pos.0 < cave[0].len() && new_pos.1 < cave.len() {
                match cave[new_pos.1][new_pos.0] {
                    Region::Rocky => {
                        if let Tool::ClimbingGear | Tool::Torch = tool {
                            queue.insert((time+1, *new_pos, tool));
                        }
                    },
                    Region::Wet => {
                        if let Tool::Neither | Tool::ClimbingGear = tool {
                            queue.insert((time+1, *new_pos, tool));
                        }
                    },
                    Region::Narrow => {
                        if let Tool::Torch | Tool::Neither = tool {
                            queue.insert((time+1, *new_pos, tool));
                        }
                    },
                };
            }
        }
    }
    output
}

fn main() {
    let (depth, target) = parse_input("input.txt");
    let cave = generate_cave(depth, target);
    let output = get_fastest_time(&cave, (0, 0), target);
    println!("Fastest time: {:?}", output);
}

