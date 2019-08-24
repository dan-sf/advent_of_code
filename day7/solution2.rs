use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

fn create_graph() -> (HashMap<char, HashSet<char>>, HashSet<char>) {
    let mut output_map: HashMap<char, HashSet<char>> = HashMap::new();
    let mut output_set: HashSet<char> = HashSet::new();

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    for line in reader.lines() {
        let line: Vec<String> = line.unwrap().split(" ").map(|l| l.to_string()).collect();
        let node = line[7].chars().collect::<Vec<char>>()[0];
        output_set.insert(node);

        let dep = line[1].chars().collect::<Vec<char>>()[0];
        output_set.insert(dep);

        let node_entry = output_map.entry(node).or_insert(HashSet::new());
        node_entry.insert(dep);
    }

    (output_map, output_set)
}

// Here we should get as many nodes as we can in relation to the workers
fn work_on_ready_tasks(graph: &HashMap<char, HashSet<char>>, total_nodes: &HashSet<char>, work: &mut Work) -> char {
    let mut can_run: Vec<char> = Vec::new();
    for n in total_nodes.iter() {
        if !graph.contains_key(n) {
            can_run.push(*n);
        } else if graph[n].is_empty() {
            can_run.push(*n);
        }
    }

    if can_run.len() == 0 {
        return false;
    }

    // While we have items in can run, we should iterate on the Work/Workers and increment time until can run is done.
    can_run.sort();
    while can_run.len() > 0 {
        if work.has_available_worker() {
            work.give_task(can_run.pop(0));
        }
        else {
            work.work_until_worker_available();
        }
    }
    true
}


fn visit_node(graph: &mut HashMap<char, HashSet<char>>, node: char) {
    graph.remove(&node);
    for (_, deps) in graph.iter_mut() {
        deps.remove(&node);
    }
}


struct Worker {
    idle: bool,
    step: Option<char>,
    seconds_left: i32,
}

struct Work {
    workers: Vec<Worker>,
    total_time: i32,
}

// get_working_set -> ['a','b','c', ...] HashSet
// new -> init the workers
// 

fn main() {
    let mut output: Vec<char> = Vec::new();
    let (mut graph, mut total_nodes) = create_graph();
    while !graph.is_empty() {

        work_on_tasks




        let node = get_next_node(&graph, &total_nodes);
        output.push(node);
        total_nodes.remove(&node);
        visit_node(&mut graph, node);
    }
    let mut order = String::new();
    let _ = output.iter().map(|n| order.push(*n)).collect::<Vec<_>>();
    println!("Instruction order: {}", order);
}

