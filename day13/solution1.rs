use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::cmp::Ordering;

// Here we load the entire grid of tracks into a matrix of enums containing the various location
// types. As we see carts we load those into a vec which we use to move the carts forward based on
// the cart state we read in

#[derive(Debug)]
enum Location {
    Empty, // -> ' '

    LeftRight, // -> '-'
    UpDown, // -> '|'

    TurnRight, // -> '/'
    TurnDown, // -> '\'

    Intersection, // -> '+'
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
    location: (usize, usize),
    intersection_action: IntersectionAction,
}

impl Cart {
    // Move the cart in which ever direction it is currently pointed in
    fn move_it(&mut self) {
        match self.direction {
            Direction::Up => {
                self.location.0 -= 1;
            },
            Direction::Down => {
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

// Implement ordering for carts so we can sort them based on location
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

// Generate cart struct given a char direction and a location
fn get_cart(ch_dir: char, location: (usize, usize)) -> Cart {
    let mut direction = Direction::Up; // Init the direction
    if ch_dir == '^' {
        direction = Direction::Up;
    } else if ch_dir == 'v' {
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

fn parse_input(path: &str) -> (Vec<Vec<Location>>, Vec<Cart>, HashSet<(usize, usize)>) {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut grid: Vec<Vec<Location>> = Vec::new();
    let mut carts: Vec<Cart> = Vec::new();
    let mut cart_locations: HashSet<(usize, usize)> = HashSet::new();

    for (r, line) in reader.lines().enumerate() {
        let mut row: Vec<Location> = Vec::new();
        for (c, ch) in line.unwrap().chars().enumerate() {
            if ch == ' ' {
                row.push(Location::Empty);
            } else if ch == '-' || ch == '>' || ch == '<' {
                row.push(Location::LeftRight);
                if ch == '>' || ch == '<' {
                    carts.push(get_cart(ch, (r, c)));
                    cart_locations.insert((r, c));
                }
            } else if ch == '|' || ch == 'v' || ch == '^' {
                row.push(Location::UpDown);
                if ch == 'v' || ch == '^' {
                    carts.push(get_cart(ch, (r, c)));
                    cart_locations.insert((r, c));
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
    (grid, carts, cart_locations)
}

fn get_first_crash(grid: Vec<Vec<Location>>, mut carts: Vec<Cart>, mut cart_locations: HashSet<(usize, usize)>) -> (usize, usize) {
    loop { // Loop/tick until we see a crash
        for cart in carts.iter_mut() {
            cart_locations.remove(&cart.location);
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
            if cart_locations.contains(&cart.location) {
                return cart.location;
            }
            cart_locations.insert(cart.location);
        }
        carts.sort();
    }
}

// Convert row/col tuples to x/y tuples
fn to_xy(row_col: (usize, usize)) -> (usize, usize) {
    (row_col.1, row_col.0)
}

fn main() {
    let (grid, carts, cart_locations) = parse_input("input.txt");
    println!("First crash location: {:?}", to_xy(get_first_crash(grid, carts, cart_locations)));
}

