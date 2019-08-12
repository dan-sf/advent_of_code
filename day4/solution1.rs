use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let mut guard_notes: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    let input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");
    let reader = io::BufReader::new(input);

    let mut lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
    lines.sort();

    let mut i = 0;

    for line in lines.iter() {
        let date = &line[1..11];
        let minute = &line[12..17];
        let action = &line[19..];
        println!("{}, {}, {}", date, minute, action);
        let parts: Vec<&str> = line.split([' ', ']'].as_ref()).collect();
        println!("{}", line);
        println!("{:?}", parts);
        if i > 10 {
            break;
        }
        i += 1;

        if action.starts_with("Guard") {
            let mut guard_id = parts[4].to_string();
            guard_id.remove(0);

            if !guard_notes.contains_key(&guard_id) {
                let mut sleep_awake: HashMap<String, Vec<String>> = HashMap::new();
                sleep_awake.insert(String::from("sleep_start"), vec![]);
                sleep_awake.insert(String::from("awake_start"), vec![]);
                guard_notes.insert(
                    guard_id,
                    sleep_awake,
                );
            } else {
                if action.starts_with("Falls") {
                    guard_notes.get_mut(&guard_id).unwrap().get_mut("sleep_start").unwrap().push(minute.to_string());
                } else {
                    guard_notes.get_mut(&guard_id).unwrap().get_mut("awake_start").unwrap().push(minute.to_string());
                }
            }
        }
    }

    struct Times {
        minutes: HashMap<i32, i32>,
        total: i32,
    }

    let mut guard_totals: HashMap<String, Times> = HashMap::new();

    for gid in guard_notes.keys() {
        if !guard_totals.contains_key(gid) {
            guard_totals.insert(gid.to_string(), Times { minutes: HashMap::new(), total: 0 });
        }
        println!("{}", gid);
    }

}
