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
        _ => unreachable!(),
    }
}

// Get split indices for the parentheses parsing
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
            initiative: initiative,
            group_type: if push_immune { GroupType::ImmuneSystem } else { GroupType::Infection },
        };

        if push_immune {
            immune_system.push(group);
        } else {
            infection.push(group);
        }

    }

    (immune_system, infection)
}

// Get a specific groups target selection
fn get_selection(select_group: &Group, target_groups: &Vec<Group>, filter: &HashSet<usize>) -> Option<usize> {
    let mut output: Option<usize> = None;
    let mut max_damage = 0;
    let mut max_initiative = 0;
    let mut max_ep = 0;

    if select_group.units > 0 {
        for (i, target) in target_groups.iter().enumerate() {
            if target.immunities.contains(&select_group.attack_type) || filter.contains(&i) || target.units <= 0 {
                continue;
            }

            let target_ep = target.units * target.damage;
            let attack_damage = get_damage(select_group, target);

            if attack_damage > max_damage {
                max_damage = attack_damage;
                max_initiative = target.initiative;
                max_ep = target_ep;
                output = Some(i);
            } else if attack_damage == max_damage {
                if target_ep > max_ep {
                    max_initiative = target.initiative;
                    max_ep = target_ep;
                    output = Some(i);
                } else if target_ep == max_ep && target.initiative > max_initiative {
                    max_initiative = target.initiative;
                    output = Some(i);
                }
            }
        }
    }
    output
}

// Generate index selections for the input group lists given
fn run_selections(select_groups: &Vec<Group>, target_groups: &Vec<Group>) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = vec![];
    let mut filter: HashSet<usize> = HashSet::new();

    for (i, sg) in select_groups.iter().enumerate() {
        if let Some(target_index) = get_selection(sg, target_groups, &filter) {
            filter.insert(target_index);
            output.push((i, target_index));
        }
    }
    output
}

// Return damage for a source group to a target group
fn get_damage(source: &Group, target: &Group) -> i32 {
    if target.immunities.contains(&source.attack_type) {
        return 0;
    }
    let mut multiplier = 1;
    if target.weaknesses.contains(&source.attack_type) {
        multiplier = 2;
    }
    return multiplier * source.damage * source.units;
}

// Return if all units are dead for a given list of groups
fn all_dead(groups: &Vec<Group>) -> bool {
    for group in groups.iter() {
        if group.units > 0 {
            return false;
        }
    }
    true
}

fn main() {
    let (mut immune_system, mut infection) = parse_input("input.txt");

    loop {
        // Order by largest ep then initiative
        infection.sort_by_key(|g| (-g.units * g.damage, -g.initiative));
        immune_system.sort_by_key(|g| (-g.units * g.damage, -g.initiative));

        // Get the attacking order of all the groups
        let mut attack_order: Vec<(i32, usize, usize, GroupType)> = vec![];
        let selection_indices = run_selections(&immune_system, &infection);
        for (im_index, in_index) in selection_indices.iter() {
            attack_order.push((immune_system[*im_index].initiative, *im_index, *in_index, immune_system[*im_index].group_type));
        }
        let selection_indices = run_selections(&infection, &immune_system);
        for (in_index, im_index) in selection_indices.iter() {
            attack_order.push((infection[*in_index].initiative, *in_index, *im_index, infection[*in_index].group_type));
        }
        attack_order.sort_by_key(|k| -k.0); // Reverse sort by initiative

        // Run the attacks
        for attack in attack_order.iter() {
            let (_init, attack_ind, target_ind, group_type) = attack;
            match group_type {
                GroupType::ImmuneSystem => {
                    let units_killed = get_damage(&immune_system[*attack_ind],
                                                  &infection[*target_ind]) / infection[*target_ind].hit_points;
                    infection[*target_ind].units -= units_killed;
                },
                GroupType::Infection => {
                    let units_killed = get_damage(&infection[*attack_ind],
                                                  &immune_system[*target_ind]) / immune_system[*target_ind].hit_points;
                    immune_system[*target_ind].units -= units_killed;
                },
            };
        }

        if all_dead(&infection) {
            println!("Immune system wins, units left: {}", immune_system.iter().map(|g| g.units).sum::<i32>());
            break;
        }

        if all_dead(&immune_system) {
            println!("Infection wins, units left: {}", infection.iter().map(|g| g.units).sum::<i32>());
            break;
        }
    }
}

