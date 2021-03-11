use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

struct Times {
    minutes: Vec<i32>,
    total: i32,
}

impl Times {
    fn new() -> Times {
        Times {
            minutes: vec![0;60],
            total: 0
        }
    }
}

fn get_parts(line: &str) -> (&str, &str, &str) {
    let date = &line[1..11];
    let minute = &line[12..17];
    let action = &line[19..];
    (date, minute, action)
}

fn main() {
    let mut guard_notes: HashMap<String, Times> = HashMap::new();

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
    lines.sort();

    let mut current_guard = String::from("");
    let mut lines_iter = lines.iter();
    while let Some(line) = lines_iter.next() {
        let (_date, minute, action) = get_parts(&line);
        let parts: Vec<&str> = line.split([' ', ']'].as_ref()).collect();

        if action.starts_with("Guard") {
            let mut guard_id = parts[4].to_string();
            guard_id.remove(0);
            current_guard = guard_id.clone();

            if !guard_notes.contains_key(&guard_id) {
                guard_notes.insert(
                    guard_id,
                    Times::new(),
                );
            }
        } else {
            let start = minute[3..].parse::<i32>().unwrap();
            let next_line = lines_iter.next().unwrap();
            let (_date, minute, _action) = get_parts(&next_line);
            let end = minute[3..].parse::<i32>().unwrap();
            let local_total = end - start;

            // Add to guard total
            let mut guard_time = guard_notes.get_mut(&current_guard).unwrap();
            guard_time.total += local_total;

            // Add to each minute total
            for m in start..end {
                guard_time.minutes[m as usize] += 1;
            }
        }
    }

    // Get max total guard id
    let mut max_total = 0;
    let mut max_gid = String::new();
    for gid in guard_notes.keys() {
        if guard_notes[gid].total > max_total {
            max_total = guard_notes[gid].total;
            max_gid = gid.to_string();
        }
    }

    // Get max min for that guard
    let mut max_min = -1;
    let mut max_min_sum = -1;
    for (index, min) in guard_notes[&max_gid].minutes.iter().enumerate() {
        if guard_notes[&max_gid].minutes[*min as usize] > max_min_sum {
            max_min_sum = guard_notes[&max_gid].minutes[*min as usize];
            max_min = index as i32;
        }
    }

    println!("Guard id: {}, Minute: {}, Product: {}", max_gid, max_min, max_min * max_gid.parse::<i32>().unwrap());
}

