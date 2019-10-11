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

#[derive(Debug, Clone, Copy)]
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

fn or_match(input_location: Location, location_a: Location, location_b: Location) -> bool {
    if let input_location = location_a {
        return true;
    }
    if let input_location = location_b {
        return true;
    }
    false
}

fn make_level_flowing(graph: &mut Vec<Vec<Location>>, left: bool, point: Point) -> Option<Point> {
    fn sum_it(a: usize, b: usize, minus: bool) -> usize {
        if minus {
            return a - b;
        }
        a + b
    }

    let mut output = None;
    let mut direction = 0;
    loop {
        let pattern = (
            Location::WaterSettled,
            Location::Sand,
            Location::Clay,
            Location::Sand
        );
        let one_past_x = sum_it(point.x, 1, left);
        let check = (
            graph[point.y][sum_it(one_past_x, direction, left)],
            graph[point.y][sum_it(point.x, direction, left)],
            graph[point.y+1][sum_it(point.x, direction, left)],
            graph[point.y+1][sum_it(one_past_x, direction, left)]
        );
        if let pattern = check {
            output = Some(Point { y: point.y, x: sum_it(point.x, direction, left) });
            break;
        }
        if let Location::Clay = graph[point.y][sum_it(point.x, direction, left)] {
            break;
        }

        println!("HERE");

        graph[point.y][sum_it(point.x, direction, left)] = Location::WaterFlowing;
        direction += 1;
    }

    output
}

fn flood(graph: &mut Vec<Vec<Location>>, point: Point) {
    if point.y+1 == graph.len() {
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
    let mut overflow = false;

    // Fill right
    if let Location::WaterSettled = graph[point.y+1][point.x] {
        graph[point.y][point.x] = Location::WaterSettled;
        let mut right = 0;

        //while let Location::WaterSettled = graph[point.y+1][point.x+right] {
        while or_match(graph[point.y+1][point.x+right], Location::WaterSettled, Location::Clay) {
            _debug_graph_print(&graph);

            if let Location::Clay = graph[point.y][point.x+right] {
                break;
            }

            if let (Location::WaterSettled, Location::Sand, Location::Clay, Location::Sand) = 
                (graph[point.y][point.x+right-1], graph[point.y][point.x+right], graph[point.y+1][point.x+right], graph[point.y+1][point.x+right+1]) {
                water_drop_locations.push(Point { y: point.y, x: point.x+right });
                //right -= 1;


                let start_at = Point { y: point.y, x: point.x+right };
                let start_at2 = Point { y: point.y, x: point.x+right };
        println!("start: {:?}, gp: {:?}", start_at, graph[start_at.y][start_at.x]);
                make_level_flowing(graph, true, start_at);
        println!("start: {:?}, gp: {:?}", start_at2, graph[start_at2.y][start_at2.x]);

                //while let Location::WaterSettled = graph[point.y][point.x+right] {
                //    graph[point.y][point.x+right] = Location::WaterFlowing;
                //    if right > 0 {
                //        right -= 1;
                //    }
                //}
                break;
            }
            graph[point.y][point.x+right] = Location::WaterSettled;
            right += 1;
        }
        //while let Location::Clay = graph[point.y+1][point.x+right] {
        //    if let Location::Clay = graph[point.y][point.x+right] {
        //        break;
        //    }
        //    graph[point.y][point.x+right] = Location::WaterSettled;
        //    right += 1;
        //}
    }

    //_debug_graph_print(&graph);

    //let mut right = 0;
    //while let Location::WaterSettled = graph[point.y+1][point.x+right] {
    //    if let Location::Sand = graph[point.y][point.x+right+1] {
    //        if let (Location::Clay, Location::Sand, Location::Sand) = (graph[point.y+1][point.x+right+1].clone(), graph[point.y][point.x+right+2].clone(), graph[point.y+1][point.x+right+2].clone()) {
    //            graph[point.y][point.x] = Location::WaterFlowing;
    //            graph[point.y][point.x+right+1] = Location::WaterFlowing;

    //            //_debug_graph_print(&graph);

    //            water_drop_locations.push(Point { y: point.y, x: point.x+right+2 });
    //            while right > 0 {
    //                graph[point.y][point.x+right] = Location::WaterFlowing;
    //                right -= 1;
    //            }
    //            while let Location::WaterSettled = graph[point.y+1][point.x-right] {
    //                graph[point.y][point.x-right] = Location::WaterFlowing;
    //                right += 1;
    //            }
    //            //_debug_graph_print(&graph);

    //            //println!("right: {}", right);
    //            if let Location::Sand = graph[point.y][point.x-right] {
    //                //println!("HERE: {:?}", point);
    //                if let Location::Clay = graph[point.y+1][point.x-right] {
    //                    graph[point.y][point.x-right] = Location::WaterFlowing;
    //                    water_drop_locations.push(Point { y: point.y, x: point.x-right-1 });
    //                }
    //            }

    //            overflow = true;

    //            break;
    //        }
    //    }
    //    right += 1;
    //}

    // Fill left
    overflow = true;
    if !overflow {
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
                if let (Location::Clay, Location::Sand, Location::Sand) = (graph[point.y+1][point.x-left-1].clone(), graph[point.y][point.x-left-2].clone(), graph[point.y+1][point.x-left-2].clone()) {
                //if let Location::Clay = graph[point.y+1][point.x-left-1] {
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
    }

    if water_drop_locations.len() > 0 {
    //println!("Water drops: {:?}", water_drop_locations);
    }

    for p in water_drop_locations.iter() {
        flood(graph, Point { x: p.x, y: p.y });
    }
}

fn get_water_count(graph: &Vec<Vec<Location>>) -> usize {
    graph.iter().flat_map(
        |v| v.iter().map(
            |l| if let Location::WaterSettled | Location::WaterFlowing = l { 1 } else { 0 })
        ).fold(0, |a,b| a+b)
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
    _debug_graph_print(&graph);
    println!("Water count: {}", get_water_count(&graph) - 1);
}

