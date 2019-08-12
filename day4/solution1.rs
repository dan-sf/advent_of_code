use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

struct Times {
    minutes: HashMap<i32, i32>,
    total: i32,
}

impl Times {
    fn new() -> Times {
        let mut min_map: HashMap<i32, i32> = HashMap::new();
        for i in 0..60 {
            min_map.insert(i, 0);
        }
        Times {
            minutes: min_map,
            total: 0
        }
    }
}

fn main() {
    let mut guard_notes: HashMap<String, Times> = HashMap::new();

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
    lines.sort();

    let mut i = 0;

    fn get_parts(line: &str) -> (&str, &str, &str) {
        let date = &line[1..11];
        let minute = &line[12..17];
        let action = &line[19..];
        (date, minute, action)
    }

    let mut current_guard = String::from("");
    let mut lines_iter = lines.iter();
    while let Some(line) = lines_iter.next() {
        let (date, minute, action) = get_parts(&line);
        println!("{}, {}, {}", date, minute, action);
        let parts: Vec<&str> = line.split([' ', ']'].as_ref()).collect();
        println!("{}", line);
        println!("{:?}", parts);
        if i > 10 {
            //break;
        }
        i += 1;

        if action.starts_with("Guard") {
            let mut guard_id = parts[4].to_string();
            guard_id.remove(0);
            current_guard = guard_id.clone();
            println!("current_guard: {}", current_guard);

            if !guard_notes.contains_key(&guard_id) {
                guard_notes.insert(
                    guard_id,
                    Times::new(),
                );
            }
        } else {
            let start = minute[3..].parse::<i32>().unwrap();
            let next_line = lines_iter.next().unwrap();
            let (date, minute, action) = get_parts(&next_line);
            let end = minute[3..].parse::<i32>().unwrap();
            let local_total = end - start;

            let mut guard_time = guard_notes.get_mut(&current_guard).unwrap();
            guard_time.total += local_total;

            for m in start..end {
                let min_val = guard_time.minutes.get_mut(&m).unwrap();
                *min_val += 1;
            }
        }
    }
}
