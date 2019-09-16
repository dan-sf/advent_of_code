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
struct Goblin {
    health: i32,
    location: (usize, usize),
}

impl Goblin {
    fn new(location: (usize, usize)) -> Goblin {
        Goblin {
            health: 200,
            location: location,
        }
    }
}

#[derive(Debug)]
struct Elf {
    health: i32,
    location: (usize, usize),
}

impl Elf {
    fn new(location: (usize, usize)) -> Elf {
        Elf {
            health: 200,
            location: location,
        }
    }
}

fn parse_input(path: &str) -> (Vec<Vec<Location>>, HashMap<(usize, usize), Cart>) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut grid: Vec<Vec<Location>> = Vec::new();
    let mut cart_map: HashMap<(usize, usize), Cart> = HashMap::new();

    for (r, line) in reader.lines().enumerate() {
        let mut row: Vec<Location> = Vec::new();
        for (c, ch) in line.unwrap().chars().enumerate() {
            if ch == ' ' {
                row.push(Location::Empty);
            } else if ch == '-' || ch == '>' || ch == '<' {
                row.push(Location::LeftRight);
                if ch == '>' || ch == '<' {
                    cart_map.insert((r,c), get_cart(ch, (r, c)));
                }
            } else if ch == '|' || ch == 'v' || ch == '^' {
                row.push(Location::UpDown);
                if ch == 'v' || ch == '^' {
                    cart_map.insert((r,c), get_cart(ch, (r, c)));
                }
            } else if ch == '/' {
                row.push(Location::TurnRight);
            } else if ch == '\\' {
                row.push(Location::TurnDown);
            } else if ch == '+' {
                row.push(Location::Intersection);
            }
        }
        grid.push(row);
    }
    (grid, cart_map)
}

fn get_last_remaining_cart(grid: Vec<Vec<Location>>, mut cart_map: HashMap<(usize, usize), Cart>) -> (usize, usize) {
    loop { // Loop/tick until there is one cart left
        let mut keys: Vec<(usize, usize)> = cart_map.keys().map(|k| k.clone()).collect();
        keys.sort();
        for key in keys {
            if cart_map.contains_key(&key) {
                let mut cart = cart_map.remove(&key).unwrap();

                match grid[cart.location.0][cart.location.1] {
                    Location::TurnRight => {
                        match cart.direction {
                            Direction::Up => {
                                cart.direction = Direction::Right;
                            },
                            Direction::Down => {
                                cart.direction = Direction::Left;
                            },
                            Direction::Left => {
                                cart.direction = Direction::Down;
                            },
                            Direction::Right => {
                                cart.direction = Direction::Up;
                            },
                        }
                    },
                    Location::TurnDown => {
                        match cart.direction {
                            Direction::Left => {
                                cart.direction = Direction::Up;
                            },
                            Direction::Right => {
                                cart.direction = Direction::Down;
                            },
                            Direction::Up => {
                                cart.direction = Direction::Left;
                            },
                            Direction::Down => {
                                cart.direction = Direction::Right;
                            },
                        }
                    },
                    Location::Intersection => {
                        match cart.direction {
                            Direction::Up => {
                                match cart.intersection_action {
                                    IntersectionAction::Left => {
                                        cart.direction = Direction::Left;
                                        cart.intersection_action = IntersectionAction::Straight;
                                    },
                                    IntersectionAction::Straight => {
                                        cart.intersection_action = IntersectionAction::Right;
                                    },
                                    IntersectionAction::Right => {
                                        cart.direction = Direction::Right;
                                        cart.intersection_action = IntersectionAction::Left;
                                    },
                                }
                            },
                            Direction::Down => {
                                match cart.intersection_action {
                                    IntersectionAction::Left => {
                                        cart.direction = Direction::Right;
                                        cart.intersection_action = IntersectionAction::Straight;
                                    },
                                    IntersectionAction::Straight => {
                                        cart.intersection_action = IntersectionAction::Right;
                                    },
                                    IntersectionAction::Right => {
                                        cart.direction = Direction::Left;
                                        cart.intersection_action = IntersectionAction::Left;
                                    },
                                }
                            },
                            Direction::Left => {
                                match cart.intersection_action {
                                    IntersectionAction::Left => {
                                        cart.direction = Direction::Down;
                                        cart.intersection_action = IntersectionAction::Straight;
                                    },
                                    IntersectionAction::Straight => {
                                        cart.intersection_action = IntersectionAction::Right;
                                    },
                                    IntersectionAction::Right => {
                                        cart.direction = Direction::Up;
                                        cart.intersection_action = IntersectionAction::Left;
                                    },
                                }
                            },
                            Direction::Right => {
                                match cart.intersection_action {
                                    IntersectionAction::Left => {
                                        cart.direction = Direction::Up;
                                        cart.intersection_action = IntersectionAction::Straight;
                                    },
                                    IntersectionAction::Straight => {
                                        cart.intersection_action = IntersectionAction::Right;
                                    },
                                    IntersectionAction::Right => {
                                        cart.direction = Direction::Down;
                                        cart.intersection_action = IntersectionAction::Left;
                                    },
                                }
                            },
                        }
                    },
                    Location::LeftRight => { },
                    Location::UpDown => { },
                    Location::Empty => {
                        // Carts should never be able to reach an empty location so returning debug
                        // bogus location
                        return (1234, 1234);
                    },
                };
                cart.move_it();

                // Detect crash
                if cart_map.contains_key(&cart.location) {
                    cart_map.remove(&cart.location);
                } else {
                    cart_map.insert(cart.location, cart);
                }
            }
        }

        // Return if there is only 1 cart at the end of the tick
        if cart_map.len() == 1 {
            for (k, _) in cart_map.drain() {
                return k;
            }
        }
    }
}

// Output what the grid currently looks like
fn _debug_grid_print(grid: &Vec<Vec<Location>>, cart_map: &HashMap<(usize, usize), Cart>) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if cart_map.contains_key(&(row, col)) {
                let cart = cart_map.get(&(row, col)).unwrap();
                match cart.direction {
                    Direction::Up => {
                        print!("^");
                    },
                    Direction::Down => {
                        print!("v");
                    },
                    Direction::Left => {
                        print!("<");
                    },
                    Direction::Right => {
                        print!(">");
                    },
                };
            } else {
                match grid[row][col] {
                    Location::Empty => {
                        print!(" ");
                    },
                    Location::LeftRight => {
                        print!("-");
                    },
                    Location::UpDown => {
                        print!("|");
                    },
                    Location::TurnRight => {
                        print!("/");
                    },
                    Location::TurnDown => {
                        print!("\\");
                    },
                    Location::Intersection => {
                        print!("+");
                    },
                };
            }
        }
        print!("\n");
    }
}

// Convert row/col tuples to x/y tuples
fn to_xy(row_col: (usize, usize)) -> (usize, usize) {
    (row_col.1, row_col.0)
}

fn main() {
    let (grid, cart_map) = parse_input("input.txt");
    println!("Last cart location: {:?}", to_xy(get_last_remaining_cart(grid, cart_map)));
}


