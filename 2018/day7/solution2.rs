use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

// Create worker and work structs to keep track of the working state as we iterate over the tasks

#[derive(Clone)]
struct Worker {
    idle: bool,
    step: Option<char>,
    seconds_left: i32,
}

impl Worker {
    fn new() -> Worker {
        Worker {
            idle: true,
            step: None,
            seconds_left: 0,
        }
    }
}

struct Work {
    workers: Vec<Worker>,
    total_time: i32,
}

impl Work {
    fn new() -> Work {
        Work {
            workers: vec![Worker::new();5],
            total_time: 0,
        }
    }

    fn doing_work(&self) -> bool {
        // Return true if any worker is busy
        for w in self.workers.iter() {
            if !w.idle {
                return true;
            }
        }
        false
    }

    fn workers_available(&self) -> bool {
        // Return true if any worker is available
        for w in self.workers.iter() {
            if w.idle {
                return true;
            }
        }
        false
    }

    fn increment_workers(&mut self, graph: &mut HashMap<char, HashSet<char>>, total_nodes: &mut HashSet<char>) {
        // Move the workers forward that have work by counting down the time spent on the worker's
        // current step if it has one. Also, finish the worker's task once it has spent all time
        // needed on it
        for w in self.workers.iter_mut() {
            // If the second_left is 1, we know that this iteration is the last one needed to
            // finish the task. So we can free up the worker, if we went to 0 we would be double
            // counting
            if w.seconds_left > 1 {
                w.seconds_left -= 1;
            } else {
                w.idle = true;
                // If there is a task on this worker we should finish it by removing the node from
                // the set of nodes that need to be worked on
                if let Some(n) = w.step {
                    remove_node(graph, total_nodes, n);
                }
                w.step = None;
            }
        }
    }

    fn give_task(&mut self, task: char) {
        // Take a task (a char step) and give it to an available worker. We also set the amount of
        // time that is needed to finish the step here
        for w in self.workers.iter_mut() {
            if w.idle {
                w.idle = false;
                w.step = Some(task);
                // Ascii 'A' is 65, so we minus 64 so for 'A' we would get 1 + 60, 'B' 2 + 60, and
                // so on
                w.seconds_left = task as i32 - 64 + 60;
                break;
            }
        }
    }

    fn working_tasks(&self) -> HashSet<char> {
        // Loop through the workers and check to see what is currently being worked on
        let mut output = HashSet::new();
        for w in self.workers.iter() {
            if let Some(n) = w.step {
                output.insert(n.clone());
            }
        }
        output
    }

    // @Debug: Function used to help track down an off by one error in increment_workers function.
    // This debug print was critical in being able to solve the problem, so I'm leaving it in
    fn _print_worker_sec(&self) {
        println!("{:?}", self.workers.iter().map(|w| (w.seconds_left, w.step)).collect::<Vec<(i32, Option<char>)>>());
    }
}

fn create_graph() -> (HashMap<char, HashSet<char>>, HashSet<char>) {
    // Here we create a HashMap of the step to a set of that steps dependencies. We also output a
    // set of all steps
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

fn give_workers_work(graph: &mut HashMap<char, HashSet<char>>, total_nodes: &mut HashSet<char>, work: &mut Work) {
    // Loop through all nodes checking to see which ones are ready to be picked up by workers
    let mut can_run: Vec<char> = Vec::new();
    for n in total_nodes.iter() {
        if !graph.contains_key(n) {
            can_run.push(*n);
        } else if graph[n].is_empty() {
            can_run.push(*n);
        }
    }

    // If no steps are available to run we exit early
    if can_run.len() == 0 {
        return;
    }

    // While there are tasks to run and workers available, send the tasks to the open workers
    can_run.sort();
    while can_run.len() > 0 && work.workers_available() {
        let task = can_run.remove(0);
        // We should only add the task to the worker if its not already being worked on. The nodes
        // do not get removed from the graph (or total_nodes) until a worker has finished its work
        // for the step
        if !work.working_tasks().contains(&task) {
            work.give_task(task.clone());
        }
    }
}

fn remove_node(graph: &mut HashMap<char, HashSet<char>>, total_nodes: &mut HashSet<char>, node: char) {
    // Remove the node/step from the graph and total_nodes
    graph.remove(&node);
    for (_, deps) in graph.iter_mut() {
        deps.remove(&node);
    }

    total_nodes.remove(&node);
}

fn main() {
    let mut work = Work::new();
    let (mut graph, mut total_nodes) = create_graph();
    while !graph.is_empty() || work.doing_work() {
        if work.workers_available() {
            give_workers_work(&mut graph, &mut total_nodes, &mut work);
        }
        // work._print_worker_sec();
        work.increment_workers(&mut graph, &mut total_nodes);
        work.total_time += 1;
    }
    println!("Total time: {}", work.total_time);
}

