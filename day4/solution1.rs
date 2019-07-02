use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let gaurd_notes: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

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
            println!("YES");
        }
    //let parts: Vec<&str> = line.split(['@', ',', ':', 'x'].as_ref()).collect();
    //let (x, y, width, height) = (
    //    parts[1].trim().parse::<i32>().unwrap(),
    //    parts[2].trim().parse::<i32>().unwrap(),
    //    parts[3].trim().parse::<i32>().unwrap(),
    //    parts[4].trim().parse::<i32>().unwrap()
    //);
    //(x, y, width, height)
    }

}
