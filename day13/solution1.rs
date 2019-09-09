
// Load the entire input into a 2d grid. Have an enum with all possible types of things that could be in a grid location. Iterate through the grid ticking the carts in turn until we see a collision


// Load the grid only the tracks and what they are into a 2d matrix, when we come to carts, load those into a vec and store their state, make sure to be able to sort the carts by row,col so we can tick them in order. We might also want to store all ocupied locations in a hashset so that every time we move a cart we can check to see if the location we move to is ocupided resulting in a crash. We would also need to remove that location from the set once we move a cart


use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

use std::cmp::Ordering;

#[derive(Debug)]
enum Location {
    Empty,

    LeftRight,
    UpDown,

    TurnRight,
    TurnDown,

    Intersection,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
enum IntersectionAction {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
struct Cart {
    direction: Direction,
    location: (i32, i32), // Maybe usize here?
    intersection_action: IntersectionAction,
}

impl Cart {
    fn move_it(&mut self) {
        match self.direction {
            Direction::Up => {
                self.location.0 -= 1;
            },
            Direction::Down => {
                self.direction = Direction::Down;
                self.location.0 += 1;
            },
            Direction::Left => {
                self.location.1 -= 1;
            },
            Direction::Right => {
                self.location.1 += 1;
            },
        };
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.location.cmp(&other.location)
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}
impl Eq for Cart {}

fn get_cart(ch_dir: char, location: (i32, i32)) -> Cart {
    let mut direction = Direction::Up;
    if ch_dir == '^' {
        direction = Direction::Up;
    } else if ch_dir == 'V' {
        direction = Direction::Down;
    } else if ch_dir == '<' {
        direction = Direction::Left;
    } else if ch_dir == '>' {
        direction = Direction::Right;
    }
    Cart {
        direction: direction,
        location: location,
        intersection_action: IntersectionAction::Left,
    }
}

fn parse_input(path: &str) -> (Vec<Vec<Location>>, Vec<Cart>, HashSet<(i32, i32)>) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut grid: Vec<Vec<Location>> = Vec::new();
    let mut carts: Vec<Cart> = Vec::new();
    let mut cart_locations: HashSet<(i32, i32)> = HashSet::new();

    for (r, line) in reader.lines().enumerate() {
        let mut row: Vec<Location> = Vec::new();
        for (c, ch) in line.unwrap().chars().enumerate() {
            if ch == ' ' {
                row.push(Location::Empty);
            } else if ch == '-' || ch == '>' || ch == '<' {
                row.push(Location::LeftRight);
                if ch == '>' || ch == '<' {
                    carts.push(get_cart(ch, (r as i32, c as i32)));
                    cart_locations.insert((r as i32, c as i32));
                }
            } else if ch == '|' || ch == 'v' || ch == '^' {
                row.push(Location::UpDown);
                if ch == 'v' || ch == '^' {
                    carts.push(get_cart(ch, (r as i32, c as i32)));
                    cart_locations.insert((r as i32, c as i32));
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
    //println!("{:?}", carts);
    (grid, carts, cart_locations)
}

fn get_first_crash(grid: Vec<Vec<Location>>, mut carts: Vec<Cart>, mut cart_locations: HashSet<(i32, i32)>) -> (i32, i32) {
    loop {
        for cart in carts.iter_mut() {
            println!("{:?}", cart);
            cart_locations.remove(&cart.location);
            match grid[cart.location.0 as usize][cart.location.1 as usize] {
                Location::TurnRight => {
                    match cart.direction {
                        Direction::Up => {
                            cart.direction = Direction::Right;
                        },
                        Direction::Left => {
                            cart.direction = Direction::Down;
                        },
                        _ => {
                            println!("a");
                            return (-1, -1);
                        }
                    }
                },
                Location::TurnDown => {
                    match cart.direction {
                        Direction::Left => {
                            cart.direction = Direction::Down;
                        },
                        Direction::Up => {
                            cart.direction = Direction::Left;
                        },
                        _ => {
                            println!("b");
                            return (-1, -1);
                        }
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
                        _ => {
                            println!("c");
                            return (-1, -1);
                        }
                    }
                },
                Location::LeftRight => { },
                Location::UpDown => { },
                Location::Empty => {
                    return (-1, -1);
                },
            };
            print!("before cart: {:?} ", cart);
            cart.move_it();
            println!("after cart: {:?}", cart);

            // Detect crash
            if cart_locations.contains(&cart.location) {
                return cart.location;
            }
            cart_locations.insert(cart.location);
        }
        carts.sort();
    }
}

fn main() {
    let (grid, carts, cart_locations) = parse_input("input.txt");
    println!("First crash location: {:?}", get_first_crash(grid, carts, cart_locations));
}

