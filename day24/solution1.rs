use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;
//use std::cell::Ref


#[derive(Debug, PartialEq, Eq)]
enum AttackType {
    Bludgeoning,
    Radiation,
    Cold,
    Fire,
    Slashing,
}

#[derive(Debug, Copy, Clone)]
enum GroupType {
    ImmuneSystem,
    Infection,
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
    group_type: GroupType,
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
    let mut id: usize = 0;
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
            initiative: initiative,
            group_type: if push_immune { GroupType::ImmuneSystem } else { GroupType::Infection },
        };

        if push_immune {
            immune_system.push(group);
        } else {
            infection.push(group);
        }

        id += 1;
    }
    (immune_system, infection)
}

fn get_selection(select_group: &Group, target_groups: &Vec<Group>, filter: &HashSet<usize>) -> Option<(usize, i32)> {
    let mut targets: Vec<Group> = vec![];
    let mut output: Option<(usize, i32)> = None;
    let mut max_damage = 0;
    let mut max_initiative = 0;
    let mut max_ep = 0;

    if select_group.units > 0 {
        for (i, target) in target_groups.iter().enumerate() {
            if target.immunities.contains(&select_group.attack_type) || filter.contains(&i) || target.units <= 0 {
                continue;
            }

            let mut multiplier = 1;
            if target.weaknesses.contains(&select_group.attack_type) {
                multiplier = 2;
            }

            let target_ep = target.units * target.damage;

            let attack_damage = multiplier * select_group.damage * select_group.units;
            if attack_damage > max_damage {
                max_damage = attack_damage;
                max_initiative = target.initiative;
                max_ep = target_ep;
                output = Some((i, max_damage));
            } else if attack_damage == max_damage { // @Note: make sure to test this...
                if target_ep > max_ep {
                    max_initiative = target.initiative;
                    max_ep = target_ep;
                    output = Some((i, max_damage));
                } else if target_ep == max_ep && target.initiative > max_initiative {
                    max_initiative = target.initiative;
                    output = Some((i, max_damage));
                }
            }
        }
    }
    output
}

//fn sort_groups(mut groups: Vec<Group>) -> Vec<Group> {
fn sort_groups(groups: &mut Vec<Group>) {
    for i in 0..groups.len() {
        let base_ep = groups[i].units * groups[i].damage;
        for j in 0..(groups.len()-i) {
            let check_ep = groups[j].units * groups[j].damage;
            if (base_ep > check_ep) || (base_ep == check_ep && groups[i].initiative > groups[j].initiative) {
                let larger = groups.remove(i);
                groups.insert(j, larger);
            }
        }
    }
}

fn run_selections(select_groups: &Vec<Group>, target_groups: &Vec<Group>) -> Vec<(usize, usize, i32)> {
    let mut output: Vec<(usize, usize, i32)> = vec![];
    let mut filter: HashSet<usize> = HashSet::new();

    for (i, sg) in select_groups.iter().enumerate() {
        if let Some(target_index_damage) = get_selection(sg, target_groups, &filter) {
            let (target_index, target_damage) = target_index_damage;
            filter.insert(target_index);
            output.push((i, target_index, target_damage));
        }
    }
    output
}

//fn set_selections(select_groups: &Vec<Group>, target_groups: &Vec<Group>) -> Vec<(usize, usize)> {
//    let mut output: Vec<(usize, usize)> = vec![];
//    let mut filter: HashSet<usize> = HashSet::new();
//
//    for (i, sg) in select_groups.iter().enumerate() {
//        if let Some(target_index) = get_selection(sg, target_groups, &filter) {
//            filter.insert(target_index);
//            output.push((i, target_index));
//        }
//    }
//    output
//}

fn main() {
    let (mut immune_system, mut infection) = parse_input("input.test.txt");
    let mut count = 0;
    loop {
        let mut filter: HashSet<usize> = HashSet::new();
        sort_groups(&mut infection);
        sort_groups(&mut immune_system);

        let mut attack_order: Vec<(i32, usize, usize, i32, GroupType)> = vec![];
        let sel_indices = run_selections(&immune_system, &infection);
        for (im_index, in_index, damage) in sel_indices.iter() {
            attack_order.push((immune_system[*im_index].initiative, *im_index, *in_index, *damage, immune_system[*im_index].group_type));
        }
        let sel_indices = run_selections(&infection, &immune_system);
        for (in_index, im_index, damage) in sel_indices.iter() {
            attack_order.push((infection[*in_index].initiative, *in_index, *im_index, *damage, infection[*in_index].group_type));
        }
        attack_order.sort_by_key(|k| -k.0); // Reverse sort by initiative
        println!("attack order: {:?}", attack_order);

        for attack in attack_order.iter() {
            // here we attack ...
            let (init, attack_ind, target_ind, damage, group_type) = attack;
            match group_type {
                GroupType::ImmuneSystem => { infection[*target_ind].units -= (damage / infection[*target_ind].hit_points); println!("infection units: {}", (damage / infection[*target_ind].hit_points)); },
                GroupType::Infection => { immune_system[*target_ind].units -= (damage / infection[*target_ind].hit_points); },
            };
        }
        println!("immune system: {:?}", immune_system);
        println!("infection: {:?}", infection);
        //println!("{:?}", run_selections(&immune_system, &infection));
        if count > 10 {
            break;
        }
        count += 1;
    }
}

