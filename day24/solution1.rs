use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;


#[derive(Debug, PartialEq, Eq)]
enum AttackType {
    Bludgeoning,
    Radiation,
    Cold,
    Fire,
    Slashing,
}

#[derive(Debug)]
struct Group {
    units: i32,
    hit_points: i32,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
    attack_type: AttackType,
    damage: i32,
    initiative: i32,
}

fn get_attack_type_from_text(text: &String) -> AttackType {
    match text.as_str() {
        "bludgeoning" => AttackType::Bludgeoning,
        "radiation" => AttackType::Radiation,
        "cold" => AttackType::Cold,
        "fire" => AttackType::Fire,
        "slashing" => AttackType::Slashing,
        _ => { println!("text: {}", text); panic!() },
    }
}

fn get_paren_index(split: &Vec<String>) -> (usize, usize, usize) {
    let mut start = 0;
    let mut sep = 0;
    let mut end = 0;

    for (i, st) in split.iter().enumerate() {
        if st.as_str().starts_with("(") {
            start = i;
        }
        if st.as_str().ends_with(";") {
            sep = i;
        }
        if st.as_str().ends_with(")") {
            end = i;
        }
    }
    (start, sep, end)
}

fn parse_input(path: &str) -> (Vec<Group>, Vec<Group>) {
    let input = File::open(path)
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(input);

    let mut immune_system: Vec<Group> = vec![];
    let mut infection: Vec<Group> = vec![];

    let mut push_immune = true;
    let mut line_iter = reader.lines();
    while let Some(line) = line_iter.next() {
        let split = line.unwrap().split(" ").map(|r| r.to_string()).collect::<Vec<String>>();
        if &split[0] == "" {
            push_immune = false;
        }

        if split.len() < 3 {
            continue;
        }

        let units = split[0].parse::<i32>().unwrap();
        let hit_points = split[4].parse::<i32>().unwrap();
        let initiative = split[split.len()-1].parse::<i32>().unwrap();

        let mut does_index = 0;
        for i in 0.. {
            if &split[i] == "does" {
                does_index = i;
                break;
            }
        }

        let damage = split[does_index+1].parse::<i32>().unwrap();
        let attack_type = get_attack_type_from_text(&split[does_index+2]);

        let (start_paren, sep, end_paren) = get_paren_index(&split);

        let mut weaknesses: Vec<AttackType> = vec![];
        let mut immunities: Vec<AttackType> = vec![];

        let push_atk_types = |start: usize, end: usize, atk_list: &mut Vec<AttackType>| {
            for i in start..=end {
                let mut atk = split[i].clone();
                atk.pop();
                atk_list.push(get_attack_type_from_text(&atk));
            }
        };

        if start_paren != end_paren {
            if sep == 0 {
                if &split[start_paren] == "(weak" {
                    push_atk_types(start_paren+2, end_paren, &mut weaknesses);
                } else {
                    push_atk_types(start_paren+2, end_paren, &mut immunities);
                }
            } else {
                if &split[start_paren] == "(weak" {
                    push_atk_types(start_paren+2, sep, &mut weaknesses);
                } else {
                    push_atk_types(start_paren+2, sep, &mut immunities);
                }
                if &split[sep+1] == "weak" {
                    push_atk_types(sep+3, end_paren, &mut weaknesses);
                } else {
                    push_atk_types(sep+3, end_paren, &mut immunities);
                }
            }
        }

        let group = Group {
            units: units,
            hit_points: hit_points,
            weaknesses: weaknesses,
            immunities: immunities,
            attack_type: attack_type,
            damage: damage,
            initiative: initiative
        };

        if push_immune {
            immune_system.push(group);
        } else {
            infection.push(group);
        }
    }
    (immune_system, infection)
}

fn get_selection(select_group: &Group, target_groups: &Vec<Group>, filter: &HashSet<usize>) -> Option<usize> {
    let mut targets: Vec<Group> = vec![];
    let mut output: Option<usize> = None;
    let mut max_damage = 0;
    let mut max_initiative = 0;

    for (i, target) in target_groups.iter().enumerate() {
        if target.immunities.contains(&select_group.attack_type) || filter.contains(&i) {
            continue;
        }

        let mut multiplier = 1;
        if target.weaknesses.contains(&select_group.attack_type) {
            multiplier = 2;
        }

        let attack_damage = multiplier * select_group.damage * select_group.units;
        if attack_damage > max_damage {
            max_damage = attack_damage;
            max_initiative = target.initiative;
            output = Some(i);
        } else if attack_damage == max_damage && max_initiative < target.initiative {
            max_initiative = target.initiative;
            output = Some(i);
        }
    }
    output
}

fn main() {
    let (immune_system, infection) = parse_input("input.test.txt");
    println!("{:?}", immune_system);
    println!();
    println!("{:?}", infection);
    let mut filter: HashSet<usize> = HashSet::new();
    //filter.insert(1);
    println!("{:?}", get_selection(&immune_system[0], &infection, &filter));
}

