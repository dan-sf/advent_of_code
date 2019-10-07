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
            //print!("#");
        }, // go left and right
        Location::WaterSettled => {
            //print!("~");
        },
        Location::WaterFlowing => {
            //print!("|");
        },
    };
}

fn main() {
    let points = parse_input("input.txt");
    println!("points: {:?}", points.len());
    println!("max x: {:?}, min x: {:?}", points.iter().map(|r| r.x).max(), points.iter().map(|r| r.x).min());
    println!("max y: {:?}, min y: {:?}", points.iter().map(|r| r.y).max(), points.iter().map(|r| r.y).min());
    println!("p: {:?}", get_points("x=452", "y=1077..1087"));
    println!("p: {:?}", get_points("y=45", "x=10..15"));
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

