use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::cmp::Ordering;

#[derive(Debug)]
enum Location {
    Wall, // -> '#'
    Open, // -> '.'
}

#[derive(Debug)]
enum Faction {
    Elf,
    Goblin,
}

#[derive(Debug)]
struct Unit {
    health: i32,
    location: (usize, usize),
    faction: Faction,
}

impl Unit {
    fn new(location: (usize, usize), faction: Faction) -> Unit {
        Unit {
            health: 200,
            location: location,
            faction: faction,
        }
    }
}

fn parse_input(path: &str) -> (Vec<Vec<Location>>, BTreeMap<(usize, usize), Unit>) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut grid: Vec<Vec<Location>> = Vec::new();
    let mut unit_map: BTreeMap<(usize, usize), Unit> = BTreeMap::new();

    for (r, line) in reader.lines().enumerate() {
        let mut row: Vec<Location> = Vec::new();
        for (c, ch) in line.unwrap().chars().enumerate() {
            if ch == '#' {
                row.push(Location::Wall);
            } else if ch == '.' || ch == 'G' || ch == 'E' {
                row.push(Location::Open);
                if ch == 'E' {
                    unit_map.insert((r, c), Unit::new((r,c), Faction::Elf));
                } else if ch == 'G' {
                    unit_map.insert((r, c), Unit::new((r,c), Faction::Goblin));
                }
            }
        }
        grid.push(row);
    }
    (grid, unit_map)
}

fn get_targets(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>, unit: &Unit) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();
    for key in unit_map.keys() {
        if !same_faction(&unit.faction, &unit_map[key].faction) {
            output.push(unit_map[key].location.clone());
        }
    }
    output
}

fn is_open(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>, target: (usize, usize)) -> bool {
    if let Location::Open = grid[target.0][target.1] {
        if !unit_map.contains_key(&target) {
            return true;
        }
    }
    false
}

fn get_in_range(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>, targets: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut output: Vec<(usize, usize)> = Vec::new();
    for target in targets.iter() {
        for step in &[(1,0), (0,1)] {
            if is_open(grid, unit_map, (target.0+step.0, target.1+step.1)) {
                output.push((target.0+step.0, target.1+step.1).clone());
            }
        }
        if target.0 > 0 {
            if is_open(grid, unit_map, (target.0-1, target.1)) {
                output.push((target.0-1, target.1).clone());
            }
        }
        if target.1 > 0 {
            if is_open(grid, unit_map, (target.0, target.1-1)) {
                output.push((target.0, target.1-1).clone());
            }
        }
    }
    output
}

fn is_valid_loc(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>, loc: (usize, usize)) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    if loc.0 < rows && loc.1 < cols && !unit_map.contains_key(&loc) && is_open(grid, unit_map, loc) {
        return true;
    }
    false
}

fn can_reach(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>, start: (usize, usize), end: (usize, usize)) -> (bool, (usize, usize, usize)) {
    let mut queue: Vec<(usize, usize, usize)> = vec![(0, start.0, start.1)]; // use insert and pop for queue-likeness
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start);
    let mut count = 0;

    // @Question: Should my bfs be in reading order or something?
    while !queue.is_empty() {
        let loc = queue.pop().unwrap();
        if (loc.1, loc.2) == end {
            return (true, loc);
        }
        let new_step = (loc.0+1, loc.1+1, loc.2);
        let new_point = (new_step.1, new_step.2);
        if is_valid_loc(grid, unit_map, new_point) && !visited.contains(&new_point) {
            queue.insert(0, new_step);
            visited.insert(new_point);
        }
        let new_step = (loc.0+1, loc.1, loc.2+1);
        let new_point = (new_step.1, new_step.2);
        if is_valid_loc(grid, unit_map, new_point) && !visited.contains(&new_point) {
            queue.insert(0, new_step);
            visited.insert(new_point);
        }
        if loc.0 > 0 {
            let new_step = (loc.0+1, loc.1-1, loc.2);
            let new_point = (new_step.1, new_step.2);
            if is_valid_loc(grid, unit_map, new_point) && !visited.contains(&new_point) {
                queue.insert(0, new_step);
                visited.insert(new_point);
            }
        }
        if loc.1 > 0 {
            let new_step = (loc.0+1, loc.1, loc.2-1);
            let new_point = (new_step.1, new_step.2);
            if is_valid_loc(grid, unit_map, new_point) && !visited.contains(&new_point) {
                queue.insert(0, new_step);
                visited.insert(new_point);
            }
        }
    }
    (false, (0,0,0)) // @Question: Should I just make this an option?
}

fn get_reachable(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>, unit: &Unit, in_range: Vec<(usize, usize)>) -> Vec<(usize, usize, usize)> {
    let mut output: Vec<(usize, usize, usize)> = Vec::new();
    for loc in in_range.iter() {
        let (can_reach_it, dist_loc) = can_reach(grid, unit_map, unit.location, *loc);
        if can_reach_it {
            output.push(dist_loc.clone());
        }
    }
    output.sort();
    output
}

fn loop_through_turns(grid: Vec<Vec<Location>>, mut unit_map: BTreeMap<(usize, usize), Unit>) {
    let rows = grid.len();
    let cols = grid[0].len();
    loop {
        let keys = unit_map.keys().map(|r| r.clone()).collect::<Vec<(usize, usize)>>();
        for (row, col) in keys.iter() {
            let mut unit = unit_map.remove(&(*row, *col)).unwrap();
            let attack_location = can_attack(&grid, &unit, &unit_map);
            if let Some(loc) = attack_location {
                unit_map.get_mut(&loc).unwrap().health -= 3; // TODO: Add code to remove units
            } else {
                let targets = get_targets(&grid, &unit_map, &unit);
                println!("targets: {:?}", targets);
                let in_range = get_in_range(&grid, &unit_map, targets);
                println!("in_range: {:?}", in_range);
                let reachable = get_reachable(&grid, &unit_map, &unit, in_range); // for reach reachable we should store how far that reachable location is from the current unit
                println!("reachable: {:?}", reachable);

                if reachable.is_empty() {
                    // Do something (continue?)
                }

                let closest_reachable = reachable[0];
                println!("closest reachable: {:?}", closest_reachable);


                let mut next_point: Vec<(usize, usize, usize)> = Vec::new();
                let new_point = (unit.location.0+1, unit.location.1);
                if is_valid_loc(&grid, &unit_map, new_point) {
                    next_point.push(can_reach(&grid, &unit_map, new_point, (closest_reachable.0, closest_reachable.1)).1);
                }
                let new_point = (unit.location.0, unit.location.1+1);
                if is_valid_loc(&grid, &unit_map, new_point) {
                    next_point.push(can_reach(&grid, &unit_map, new_point, (closest_reachable.0, closest_reachable.1)).1);
                }
                if unit.location.0 > 0 {
                    let new_point = (unit.location.0-1, unit.location.1);
                    if is_valid_loc(&grid, &unit_map, new_point) {
                        next_point.push(can_reach(&grid, &unit_map, new_point, (closest_reachable.0, closest_reachable.1)).1);
                    }
                }
                if unit.location.1 > 0 {
                    let new_point = (unit.location.0, unit.location.1-1);
                    if is_valid_loc(&grid, &unit_map, new_point) {
                        next_point.push(can_reach(&grid, &unit_map, new_point, (closest_reachable.0, closest_reachable.1)).1);
                    }
                }
                println!("next_point: {:?}", next_point);

                // How do I figure out which way to move??? (I could just get closest reachable dist from all 4 points around the unit

                //let nearest = get_nearest(&grid, reachable);
                //let chosen = nearest[0];
                //let move_loc = get_move_dir(&grid, &unit);
                //move_unit(&mut unit, &grid);
            }
            unit_map.insert(unit.location.clone(), unit);
        }
        break;
    }
}

fn same_faction(a: &Faction, b: &Faction) -> bool {
    match a {
        Faction::Elf => {
            match b {
                Faction::Elf => {
                    return true;
                },
                Faction::Goblin => {
                    return false;
                },
            };
        },
        Faction::Goblin => {
            match b {
                Faction::Elf => {
                    return false;
                },
                Faction::Goblin => {
                    return true;
                },
            };
        },
    };
}

fn can_attack(grid: &Vec<Vec<Location>>, unit: &Unit, unit_map: &BTreeMap<(usize, usize), Unit>) -> Option<(usize, usize)> {
    let row = unit.location.0;
    let col = unit.location.0;
    if row > 0 && unit_map.contains_key(&(row-1, col)) && same_faction(&unit_map[&(row-1, col)].faction, &unit.faction) {
        return Some((row-1, col));
    }
    None
}

// fn get_last_remaining_cart(grid: Vec<Vec<Location>>, mut cart_map: HashMap<(usize, usize), Cart>) -> (usize, usize) {
//     loop { // Loop/tick until there is one cart left
//         let mut keys: Vec<(usize, usize)> = cart_map.keys().map(|k| k.clone()).collect();
//         keys.sort();
//         for key in keys {
//             if cart_map.contains_key(&key) {
//                 let mut cart = cart_map.remove(&key).unwrap();
// 
//                 match grid[cart.location.0][cart.location.1] {
//                     Location::TurnRight => {
//                         match cart.direction {
//                             Direction::Up => {
//                                 cart.direction = Direction::Right;
//                             },
//                             Direction::Down => {
//                                 cart.direction = Direction::Left;
//                             },
//                             Direction::Left => {
//                                 cart.direction = Direction::Down;
//                             },
//                             Direction::Right => {
//                                 cart.direction = Direction::Up;
//                             },
//                         }
//                     },
//                     Location::TurnDown => {
//                         match cart.direction {
//                             Direction::Left => {
//                                 cart.direction = Direction::Up;
//                             },
//                             Direction::Right => {
//                                 cart.direction = Direction::Down;
//                             },
//                             Direction::Up => {
//                                 cart.direction = Direction::Left;
//                             },
//                             Direction::Down => {
//                                 cart.direction = Direction::Right;
//                             },
//                         }
//                     },
//                     Location::Intersection => {
//                         match cart.direction {
//                             Direction::Up => {
//                                 match cart.intersection_action {
//                                     IntersectionAction::Left => {
//                                         cart.direction = Direction::Left;
//                                         cart.intersection_action = IntersectionAction::Straight;
//                                     },
//                                     IntersectionAction::Straight => {
//                                         cart.intersection_action = IntersectionAction::Right;
//                                     },
//                                     IntersectionAction::Right => {
//                                         cart.direction = Direction::Right;
//                                         cart.intersection_action = IntersectionAction::Left;
//                                     },
//                                 }
//                             },
//                             Direction::Down => {
//                                 match cart.intersection_action {
//                                     IntersectionAction::Left => {
//                                         cart.direction = Direction::Right;
//                                         cart.intersection_action = IntersectionAction::Straight;
//                                     },
//                                     IntersectionAction::Straight => {
//                                         cart.intersection_action = IntersectionAction::Right;
//                                     },
//                                     IntersectionAction::Right => {
//                                         cart.direction = Direction::Left;
//                                         cart.intersection_action = IntersectionAction::Left;
//                                     },
//                                 }
//                             },
//                             Direction::Left => {
//                                 match cart.intersection_action {
//                                     IntersectionAction::Left => {
//                                         cart.direction = Direction::Down;
//                                         cart.intersection_action = IntersectionAction::Straight;
//                                     },
//                                     IntersectionAction::Straight => {
//                                         cart.intersection_action = IntersectionAction::Right;
//                                     },
//                                     IntersectionAction::Right => {
//                                         cart.direction = Direction::Up;
//                                         cart.intersection_action = IntersectionAction::Left;
//                                     },
//                                 }
//                             },
//                             Direction::Right => {
//                                 match cart.intersection_action {
//                                     IntersectionAction::Left => {
//                                         cart.direction = Direction::Up;
//                                         cart.intersection_action = IntersectionAction::Straight;
//                                     },
//                                     IntersectionAction::Straight => {
//                                         cart.intersection_action = IntersectionAction::Right;
//                                     },
//                                     IntersectionAction::Right => {
//                                         cart.direction = Direction::Down;
//                                         cart.intersection_action = IntersectionAction::Left;
//                                     },
//                                 }
//                             },
//                         }
//                     },
//                     Location::LeftRight => { },
//                     Location::UpDown => { },
//                     Location::Empty => {
//                         // Carts should never be able to reach an empty location so returning debug
//                         // bogus location
//                         return (1234, 1234);
//                     },
//                 };
//                 cart.move_it();
// 
//                 // Detect crash
//                 if cart_map.contains_key(&cart.location) {
//                     cart_map.remove(&cart.location);
//                 } else {
//                     cart_map.insert(cart.location, cart);
//                 }
//             }
//         }
// 
//         // Return if there is only 1 cart at the end of the tick
//         if cart_map.len() == 1 {
//             for (k, _) in cart_map.drain() {
//                 return k;
//             }
//         }
//     }
// }

// Output what the grid currently looks like
fn _debug_grid_print(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if unit_map.contains_key(&(row, col)) {
                let unit = unit_map.get(&(row, col)).unwrap();
                match unit.faction {
                    Faction::Elf => {
                        print!("E");
                    },
                    Faction::Goblin => {
                        print!("G");
                    },
                };
            } else {
                match grid[row][col] {
                    Location::Wall => {
                        print!("#");
                    },
                    Location::Open => {
                        print!(".");
                    },
                };
            }
        }
        print!("\n");
    }
}

// // Convert row/col tuples to x/y tuples
// fn to_xy(row_col: (usize, usize)) -> (usize, usize) {
//     (row_col.1, row_col.0)
// }

fn main() {
    //let (mut grid, mut unit_map) = parse_input("input.txt");
    let (mut grid, mut unit_map) = parse_input("input.test.txt");
    loop_through_turns(grid, unit_map);

    // println!("{:?}", unit_map[&(1,14)]);
    // let mut u = unit_map.remove(&(1,14)).unwrap();
    // u.location.1 += 2;
    // unit_map.insert(u.location, u);
    // _debug_grid_print(&grid, &unit_map);

    //println!("Last cart location: {:?}", to_xy(get_last_remaining_cart(grid, cart_map)));
}

