use std::fs;
use std::io;
use std::io::BufRead;


#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum Location {
    Sand,
    Clay,
    WaterSettled,
    WaterFlowing,
}

// Expand the point coords to get each point within a given '..' range
fn get_points(point_a: &str, point_b: &str) -> Vec<Point> {
    let mut output: Vec<Point> = Vec::new();
    let split_a = point_a.split('=').collect::<Vec<&str>>();
    let (a_x_y, a_val) = (split_a[0], split_a[1]);
    let split_b = point_b.split('=').collect::<Vec<&str>>();
    let (_b_x_y, b_val) = (split_b[0], split_b[1]);
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

// Output graph and starting point
fn create_graph(points: Vec<Point>) -> (Point, Vec<Vec<Location>>) {
    let max_x = points.iter().map(|r| r.x).max().unwrap();
    let min_x = points.iter().map(|r| r.x).min().unwrap();
    let max_y = points.iter().map(|r| r.y).max().unwrap();

    let mut graph: Vec<Vec<Location>> = vec![vec![Location::Sand; max_x - min_x + 3]; max_y + 1];
    let start = Point { x: 500 - min_x + 1, y: 0 }; // Determine starting point

    for point in points.iter() {
        graph[point.y][point.x - min_x + 1] = Location::Clay;
    }

    (start, graph)
}

// For a given level of settled water, convert it to flowing water
fn convert_level_to_flowing(graph: &mut Vec<Vec<Location>>, point: Point) {
    let mut runner = 0;
    while let Location::WaterSettled = graph[point.y][point.x+runner] {
        graph[point.y][point.x+runner] = Location::WaterFlowing;
        runner += 1;
    }

    runner = 1;
    while let Location::WaterSettled = graph[point.y][point.x-runner] {
        graph[point.y][point.x-runner] = Location::WaterFlowing;
        runner += 1;
    }
}

// Create a settled layer of water at a given point, if the water flows off the edge return those
// points
fn create_settled_layer(graph: &mut Vec<Vec<Location>>, point: Point) -> (Option<Point>, Option<Point>) {
    let mut flowing_left: Option<Point> = None;
    let mut flowing_right: Option<Point> = None;

    let mut runner = 0;
    loop {
        if let Location::Clay = graph[point.y][point.x+runner] {
            break;
        }

        let check = (
            graph[point.y][point.x+runner],
            graph[point.y+1][point.x+runner],
            graph[point.y][point.x+runner+1],
            graph[point.y+1][point.x+runner+1],
        );

        // Condition for water drop point
        if let (_, Location::Clay, Location::Sand, Location::Sand) = check {
            if let Location::Sand | Location::WaterFlowing = check.0 {
                graph[point.y][point.x+runner] = Location::WaterSettled;
                flowing_right = Some(Point { y: point.y, x: point.x+runner+1 });
                break;
            }
        }

        graph[point.y][point.x+runner] = Location::WaterSettled;
        runner += 1;
    }

    runner = 0;
    loop {
        if let Location::Clay = graph[point.y][point.x-runner] {
            break;
        }

        let check = (
            graph[point.y][point.x-runner],
            graph[point.y+1][point.x-runner],
            graph[point.y][point.x-runner-1],
            graph[point.y+1][point.x-runner-1],
        );

        // Condition for water drop point
        if let (_, Location::Clay, Location::Sand, Location::Sand) = check {
            // Here we account for the case when the previous loop set the current point to water
            // settled, in that case check.0 will be settled water
            if let Location::Sand | Location::WaterFlowing | Location::WaterSettled = check.0 {
                graph[point.y][point.x-runner] = Location::WaterSettled;
                flowing_left = Some(Point { y: point.y, x: point.x-runner-1 });
                break;
            }
        }

        graph[point.y][point.x-runner] = Location::WaterSettled;
        runner += 1;
    }

    (flowing_left, flowing_right)
}

// Here we recursively flood the graph
fn flood(graph: &mut Vec<Vec<Location>>, point: Point) {
    if point.y+1 >= graph.len() { // Base case, bottom of the graph
        if let Location::Sand = graph[point.y][point.x] {
            graph[point.y][point.x] = Location::WaterFlowing;
        }
        return;
    }

    match graph[point.y][point.x] {
        Location::Sand => { 
            // Flow downwards through the sand
            graph[point.y][point.x] = Location::WaterFlowing;
            flood(graph, Point { x: point.x, y: point.y+1 });
        },
        Location::Clay => {
            return;
        },
        Location::WaterSettled => { },
        Location::WaterFlowing => {
            return;
        },
    };

    let mut water_drop_locations: Vec<Point> = Vec::new();
    if let Location::Clay | Location::WaterSettled = graph[point.y+1][point.x] {
        let (flowing_left, flowing_right) = create_settled_layer(graph, point);

        if let Some(left) = flowing_left {
            convert_level_to_flowing(graph, point);
            water_drop_locations.push(left);
        }
        if let Some(right) = flowing_right {
            convert_level_to_flowing(graph, point);
            water_drop_locations.push(right);
        }
    }

    // Recurse on downward flow points
    for p in water_drop_locations.iter() {
        flood(graph, Point { x: p.x, y: p.y });
    }
}

// Count the water given a max/min y
fn get_water_count(graph: &Vec<Vec<Location>>, min: usize, max: usize) -> usize {
    let mut output = 0;

    for i in min..=max {
        output += graph[i].iter().map(
            |l| if let Location::WaterSettled | Location::WaterFlowing = l { 1 } else { 0 }
        ).fold(0, |a,b| a+b);
    }
    output
}

fn _debug_graph_print(graph: &Vec<Vec<Location>>) {
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
    println!("");
}

fn main() {
    let points = parse_input("input.txt");
    let max_y = points.iter().map(|r| r.y).max().unwrap();
    let min_y = points.iter().map(|r| r.y).min().unwrap();
    let (start, mut graph) = create_graph(points);
    flood(&mut graph, start);
    //_debug_graph_print(&graph);
    println!("Water count: {}", get_water_count(&graph, min_y, max_y));
}

