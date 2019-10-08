use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn get_points(point_a: &str, point_b: &str) -> Vec<Point> {
    let mut output: Vec<Point> = Vec::new();
    let split_a = point_a.split('=').collect::<Vec<&str>>();
    let (a_x_y, a_val) = (split_a[0], split_a[1]);
    let split_b = point_b.split('=').collect::<Vec<&str>>();
    let (b_x_y, b_val) = (split_b[0], split_b[1]);
    if a_x_y == "x" {
        let start_end: Vec<usize> = b_val.split("..").map(|r| r.parse::<usize>().unwrap()).collect();
        for i in start_end[0]..=start_end[1] {
            output.push(Point { x: a_val.parse::<usize>().unwrap(), y: i });
        }
    } else {
        let start_end: Vec<usize> = b_val.split("..").map(|r| r.parse::<usize>().unwrap()).collect();
        for i in start_end[0]..=start_end[1] {
            output.push(Point { x: i, y: a_val.parse::<usize>().unwrap() });
        }
    }
    output
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_input(path: &str) -> Vec<Point> {
    let input = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut points: Vec<Point> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.replace(" ", "");
        let point: Vec<&str> = line.split(',').collect();
        points.extend(get_points(point[0], point[1]));
    }
    points
}

#[derive(Debug, Clone)]
enum Location {
    Sand,
    Clay,
    WaterSettled,
    WaterFlowing,
}

fn create_graph(points: Vec<Point>) -> (Point, Vec<Vec<Location>>) {
    let max_x = points.iter().map(|r| r.x).max().unwrap();
    let min_x = points.iter().map(|r| r.x).min().unwrap();
    let max_y = points.iter().map(|r| r.y).max().unwrap();

    let mut graph: Vec<Vec<Location>> = vec![vec![Location::Sand; max_x - min_x + 3]; max_y + 1];
    let start = Point { x: 500 - min_x + 1, y: 0 };

    for point in points.iter() {
        graph[point.y][point.x - min_x + 1] = Location::Clay;
    }

    (start, graph)
}

fn flood(graph: &mut Vec<Vec<Location>>, point: Point) {
    if point.y+2 == graph.len() {
        graph[point.y][point.x] = Location::WaterFlowing;
        return;
    }

    graph[point.y][point.x] = Location::WaterFlowing;
    match graph[point.y+1][point.x] {
        Location::Sand => { 
            flood(graph, Point { x: point.x, y: point.y+1 });
        },
        Location::Clay => {
            graph[point.y][point.x] = Location::WaterSettled;

            if let Location::Sand = graph[point.y][point.x+1] {
                flood(graph, Point { x: point.x+1, y: point.y });
            }

            if let Location::Sand = graph[point.y][point.x-1] {
                flood(graph, Point { x: point.x-1, y: point.y });
            }
        }, // go left and right
        Location::WaterSettled => {
            graph[point.y][point.x] = Location::WaterSettled;

            if let Location::Sand = graph[point.y][point.x+1] {
                flood(graph, Point { x: point.x+1, y: point.y });
            }

            if let Location::Sand = graph[point.y][point.x-1] {
                flood(graph, Point { x: point.x-1, y: point.y });
            }
        },
        Location::WaterFlowing => {
            //print!("|");
        },
    };


    let mut water_drop_locations: Vec<Point> = Vec::new();

    // Fill right
    if let Location::WaterSettled = graph[point.y+1][point.x] {
        graph[point.y][point.x] = Location::WaterSettled;
        let mut right = 0;
        while let Location::WaterSettled = graph[point.y+1][point.x+right] {
            if let Location::Clay = graph[point.y][point.x+right] {
                break;
            }
            graph[point.y][point.x+right] = Location::WaterSettled;
            right += 1;
        }
    }

    let mut right = 0;
    while let Location::WaterSettled = graph[point.y+1][point.x+right] {
        if let Location::Sand = graph[point.y][point.x+right+1] {
            if let Location::Clay = graph[point.y+1][point.x+right+1] {
                graph[point.y][point.x] = Location::WaterFlowing;
                graph[point.y][point.x+right+1] = Location::WaterFlowing;
                while right > 0 {
                    graph[point.y][point.x+right] = Location::WaterFlowing;
                    right -= 1;
                }
                while let Location::WaterSettled = graph[point.y+1][point.x-right] {
                    graph[point.y][point.x-right] = Location::WaterFlowing;
                    right += 1;
                }

                if let Location::Sand = graph[point.y][point.x-right] {
                    graph[point.y][point.x-right] = Location::WaterFlowing;
                    water_drop_locations.push(Point { y: point.y, x: point.x-right-1 })
                }
                break;
            }
        }
        right += 1;
    }

    // Fill left
    if let Location::WaterSettled = graph[point.y+1][point.x] {
        graph[point.y][point.x] = Location::WaterSettled;
        let mut left = 0;
        while let Location::WaterSettled = graph[point.y+1][point.x-left] {
            if let Location::Clay = graph[point.y][point.x-left] {
                break;
            }
            graph[point.y][point.x-left] = Location::WaterSettled;
            left += 1;
        }
    }

    let mut left = 0;
    while let Location::WaterSettled = graph[point.y+1][point.x-left] {
        if let Location::Sand = graph[point.y][point.x-left-1] {
            if let Location::Clay = graph[point.y+1][point.x-left-1] {
                graph[point.y][point.x] = Location::WaterFlowing;
                graph[point.y][point.x-left-1] = Location::WaterFlowing;
                water_drop_locations.push(Point { y: point.y, x: point.x-left-2 });
                while left > 0 {
                    graph[point.y][point.x-left] = Location::WaterFlowing;
                    left -= 1;
                }
                while let Location::WaterSettled = graph[point.y+1][point.x+left] {
                    graph[point.y][point.x+left] = Location::WaterFlowing;
                    left += 1;
                }

                break;
            }
        }
        left += 1;
    }

    if water_drop_locations.len() > 0 {
    println!("Water drops: {:?}", water_drop_locations);
    }
}

fn main() {
    //let points = parse_input("input.txt");
    let points = parse_input("input.test.txt");
    //println!("points: {:?}", points.len());
    //println!("max x: {:?}, min x: {:?}", points.iter().map(|r| r.x).max(), points.iter().map(|r| r.x).min());
    //println!("max y: {:?}, min y: {:?}", points.iter().map(|r| r.y).max(), points.iter().map(|r| r.y).min());
    //println!("p: {:?}", get_points("x=452", "y=1077..1087"));
    //println!("p: {:?}", get_points("y=45", "x=10..15"));
    // create graph ...
    let (start, mut graph) = create_graph(points);
    flood(&mut graph, start);
    for g in graph.iter() {
        for j in g.iter() {
            match j {
                Location::Sand => { print!("."); },
                Location::Clay => { print!("#"); },
                Location::WaterSettled => { print!("~"); },
                Location::WaterFlowing => { print!("|"); },
            }
        }
        println!("");
    }
}

