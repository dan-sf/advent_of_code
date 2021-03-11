use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::BTreeMap;

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
    attack_action: i32,
    attack_value: i32,
}

impl Unit {
    fn new(location: (usize, usize), faction: Faction, attack_value: i32) -> Unit {
        Unit {
            health: 200,
            location: location,
            faction: faction,
            attack_action: 0,
            attack_value: attack_value,
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
                    unit_map.insert((r, c), Unit::new((r,c), Faction::Elf, 3));
                } else if ch == 'G' {
                    unit_map.insert((r, c), Unit::new((r,c), Faction::Goblin, 3));
                }
            }
        }
        grid.push(row);
    }
    (grid, unit_map)
}

// Get a list of all posible targets
fn get_targets(unit_map: &BTreeMap<(usize, usize), Unit>, unit: &Unit) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();
    for key in unit_map.keys() {
        if !same_faction(&unit.faction, &unit_map[key].faction) {
            output.push(unit_map[key].location.clone());
        }
    }
    output
}

// Check if a location if open for travel
fn is_open(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>, target: (usize, usize)) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    if target.0 < rows && target.1 < cols && !unit_map.contains_key(&target) {
        if let Location::Open = grid[target.0][target.1] {
            return true;
        }
    }
    false
}

// Get in-range locations around given targets
fn get_in_range(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>, targets: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
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

// Check if a given point is reachable and also return the distance from start to end
fn can_reach_distance(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>, start: (usize, usize), end: (usize, usize)) -> (bool, (usize, usize, usize)) {
    let mut queue: Vec<(usize, usize, usize)> = vec![(0, start.0, start.1)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start);

    // Use BFS and keep track of distance
    while !queue.is_empty() {
        let loc = queue.pop().unwrap();
        if (loc.1, loc.2) == end {
            return (true, loc);
        }
        if loc.1 > 0 {
            let new_step = (loc.0+1, loc.1-1, loc.2);
            let new_point = (new_step.1, new_step.2);
            if is_open(grid, unit_map, new_point) && !visited.contains(&new_point) {
                queue.insert(0, new_step);
                visited.insert(new_point);
            }
        }
        if loc.2 > 0 {
            let new_step = (loc.0+1, loc.1, loc.2-1);
            let new_point = (new_step.1, new_step.2);
            if is_open(grid, unit_map, new_point) && !visited.contains(&new_point) {
                queue.insert(0, new_step);
                visited.insert(new_point);
            }
        }
        let new_step = (loc.0+1, loc.1, loc.2+1);
        let new_point = (new_step.1, new_step.2);
        if is_open(grid, unit_map, new_point) && !visited.contains(&new_point) {
            queue.insert(0, new_step);
            visited.insert(new_point);
        }
        let new_step = (loc.0+1, loc.1+1, loc.2);
        let new_point = (new_step.1, new_step.2);
        if is_open(grid, unit_map, new_point) && !visited.contains(&new_point) {
            queue.insert(0, new_step);
            visited.insert(new_point);
        }
    }
    (false, (0,0,0)) // It may have made more sense to use an optional here ...
}

// Get reachable points from the given in-range points
fn get_reachable(grid: &Vec<Vec<Location>>, unit_map: &BTreeMap<(usize, usize), Unit>, unit: &Unit, in_range: Vec<(usize, usize)>) -> Vec<(usize, usize, usize)> {
    let mut output: Vec<(usize, usize, usize)> = Vec::new();
    for loc in in_range.iter() {
        let (can_reach_it, dist_loc) = can_reach_distance(grid, unit_map, unit.location, *loc);
        if can_reach_it {
            output.push(dist_loc.clone());
        }
    }
    output.sort();
    output
}

// Main loop for simulating rounds
fn loop_through_turns(grid: Vec<Vec<Location>>, mut unit_map: BTreeMap<(usize, usize), Unit>) -> i32 {
    let mut rounds: i32 = 0;
    loop {
        let keys = unit_map.keys().map(|r| r.clone()).collect::<Vec<(usize, usize)>>();

        // There is a bug in the way I iterate through units, since we are looping through the keys
        // as they existed at the start of the round there is a chance that a unit who moves into a
        // spot of a removed unit, that it will take two actions. Here we keep track of the number
        // of attacks each unit takes to make sure that doesn't happen and clear them out each
        // round. There is probably a better way to do this.
        for val in unit_map.values_mut() {
            val.attack_action = 0;
        }

        // Loop through units for this round
        for (row, col) in keys.iter() {
            if !unit_map.contains_key(&(*row, *col)) {
                continue;
            }

            // Pull out the unit at it's current location so we can operate on it
            let mut unit = unit_map.remove(&(*row, *col)).unwrap();
            let attack_location = can_attack(&unit, &unit_map);

            // Attack if we can or try to move
            if let Some(loc) = attack_location {
                if unit.attack_action == 0 {
                    unit_map.get_mut(&loc).unwrap().health -= unit.attack_value;
                    if unit_map[&loc].health <= 0 {
                        // If any elf dies return -1 to binary search on
                        if let Faction::Elf = unit_map[&loc].faction {
                            return -1;
                        }
                        unit_map.remove(&loc);
                    }
                    unit.attack_action += 1;
                }
            } else {
                let targets = get_targets(&unit_map, &unit);

                // If there are no targets the battle is over, return battle score
                if targets.is_empty() {
                    let mut health_sum: i32 = unit_map.keys().map(|k| unit_map[k].health).fold(0, |a,b| a+b);
                    health_sum += unit.health;
                    //_debug_grid_print(&grid, &unit_map);
                    return health_sum * rounds;
                }

                let in_range = get_in_range(&grid, &unit_map, targets);
                let reachable = get_reachable(&grid, &unit_map, &unit, in_range);

                // If there are reachable points, move to the closest one
                if !reachable.is_empty() {
                    let closest_reachable = reachable[0];

                    // For the closest reachable point determine which direction to move in to make
                    // progress towards that point
                    let mut test_points: Vec<(usize, usize, usize)> = Vec::new();
                    let new_point = (unit.location.0+1, unit.location.1);
                    if is_open(&grid, &unit_map, new_point) {
                        let (_, dist_loc) = can_reach_distance(&grid, &unit_map, new_point, (closest_reachable.1, closest_reachable.2));
                        test_points.push((dist_loc.0, new_point.0, new_point.1));
                    }
                    let new_point = (unit.location.0, unit.location.1+1);
                    if is_open(&grid, &unit_map, new_point) {
                        let (_, dist_loc) = can_reach_distance(&grid, &unit_map, new_point, (closest_reachable.1, closest_reachable.2));
                        test_points.push((dist_loc.0, new_point.0, new_point.1));
                    }
                    if unit.location.0 > 0 {
                        let new_point = (unit.location.0-1, unit.location.1);
                        if is_open(&grid, &unit_map, new_point) {
                            let (_, dist_loc) = can_reach_distance(&grid, &unit_map, new_point, (closest_reachable.1, closest_reachable.2));
                            test_points.push((dist_loc.0, new_point.0, new_point.1));
                        }
                    }
                    if unit.location.1 > 0 {
                        let new_point = (unit.location.0, unit.location.1-1);
                        if is_open(&grid, &unit_map, new_point) {
                            let (_, dist_loc) = can_reach_distance(&grid, &unit_map, new_point, (closest_reachable.1, closest_reachable.2));
                            test_points.push((dist_loc.0, new_point.0, new_point.1));
                        }
                    }
                    test_points.sort();

                    // Move the unit
                    let point_to_move_to = (test_points[0].1, test_points[0].2);
                    unit.location = point_to_move_to;

                    // Attack if we made it to a target
                    let attack_location = can_attack(&unit, &unit_map);
                    if let Some(loc) = attack_location {
                        if unit.attack_action == 0 {
                            unit_map.get_mut(&loc).unwrap().health -= unit.attack_value;
                            if unit_map[&loc].health <= 0 {
                                // If any elf dies return -1 to binary search on
                                if let Faction::Elf = unit_map[&loc].faction {
                                    return -1;
                                }
                                unit_map.remove(&loc);
                            }
                            unit.attack_action += 1;
                        }
                    }
                }
            }

            // Don't re-insert the unit if it's health is gone
            if unit.health > 0 {
                unit_map.insert(unit.location.clone(), unit);
            }
        }
        rounds += 1;
    }
}

// Matching function to determine if two factions are equal, I feel like there should be a better
// way to do this...
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

// Check to see if a given unit can attack a unit in another faction
fn can_attack(unit: &Unit, unit_map: &BTreeMap<(usize, usize), Unit>) -> Option<(usize, usize)> {
    let row = unit.location.0;
    let col = unit.location.1;
    let mut attack_options: Vec<(i32, usize, usize)> = Vec::new();
    if row > 0 && unit_map.contains_key(&(row-1, col)) && !same_faction(&unit_map[&(row-1, col)].faction, &unit.faction) {
        attack_options.push((unit_map[&(row-1, col)].health, row-1, col));
    }
    if col > 0 && unit_map.contains_key(&(row, col-1)) && !same_faction(&unit_map[&(row, col-1)].faction, &unit.faction) {
        attack_options.push((unit_map[&(row, col-1)].health, row, col-1));
    }
    if unit_map.contains_key(&(row+1, col)) && !same_faction(&unit_map[&(row+1, col)].faction, &unit.faction) {
        attack_options.push((unit_map[&(row+1, col)].health, row+1, col));
    }
    if unit_map.contains_key(&(row, col+1)) && !same_faction(&unit_map[&(row, col+1)].faction, &unit.faction) {
        attack_options.push((unit_map[&(row, col+1)].health, row, col+1));
    }
    if attack_options.is_empty() {
        return None;
    }
    attack_options.sort();
    return Some((attack_options[0].1, attack_options[0].2));
}

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

fn main() {
    let mut start = 3;
    let mut end = 50;
    let mut guess = 0;
    let mut battle_score = 0;

    // Here we just binary search on the simulations until we get the smallest possible elf attack
    // value
    while start <= end {
        guess = (start + end) / 2;
        let (grid, mut unit_map) = parse_input("input.txt");
        for u in unit_map.values_mut() {
            if let Faction::Elf = u.faction {
                u.attack_value = guess;
            }
        }
        battle_score = loop_through_turns(grid, unit_map);
        println!("Start: {}, End: {}, Guess: {}, Battle score: {}", start, end, guess, battle_score);
        if battle_score == -1 {
            start = guess + 1;
        } else {
            end = guess - 1;
        }
    }
    println!("Battle score: {}, attack value: {}", battle_score, guess);
}

