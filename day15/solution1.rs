use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
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

fn loop_through_turns(grid: Vec<Vec<Location>>, unit_map: BTreeMap<(usize, usize), Unit>) {
    let rows = grid.len();
    let cols = grid[0].len();
    loop {
        for (row, col) in unit_map.iter() {
            let mut unit = unit_map[(row, col)];
            let attack_location = can_attack(&unit, &grid);
            if let attack_location = Some(loc) {
                unit_map[loc].health -= 3; // TODO: Add code to remove units
            } else {
                let targets = get_targets(&unit_map, (row, col), &grid);
                let in_ragne = get_in_range(targets, &grid);
                let reachable = get_reachable(in_range, &grid);
                let nearest = get_nearest(reachable, &grid);
                let chosen = nearest[0];
                let move_loc = get_move_dir(&unit, &grid);
                move_unit(&mut unit, &grid);
            }
        }
    }
}

fn can_attack(unit: &Unit, grid: &Vec<Vec<Location>>) -> Option((usize, usize)) {
    //
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
    let (mut grid, mut unit_map) = parse_input("input.txt");
    println!("{:?}", unit_map[&(1,14)]);
    let mut u = unit_map.remove(&(1,14)).unwrap();
    u.location.1 += 2;
    unit_map.insert(u.location, u);
    _debug_grid_print(&grid, &unit_map);
    //println!("Last cart location: {:?}", to_xy(get_last_remaining_cart(grid, cart_map)));
}

