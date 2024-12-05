use std::{collections::HashMap, fs::read_to_string, io};

fn part1() -> Result<(), io::Error>{
    let contents = read_to_string("input/day05.txt")?;

    let (rules,updates) = contents.split_once("\n\n").unwrap();

    let rule_map = process_rules(rules);

    let result = process_updates(updates, rule_map);

    println!("result = {result}");

    Ok(())
}

fn process_rules(rules: &str) -> HashMap<i32,Vec<i32>> {
    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rules.lines() {
        let required = rule[..2].parse::<i32>().unwrap();
        let requiree = rule[3..].parse::<i32>().unwrap();

        rule_map.entry(requiree).and_modify(|list| list.push(required)).or_default();
    }

    rule_map
}

fn process_updates(updates: &str, rule_map: HashMap<i32,Vec<i32>>) -> i32 {
    updates.lines()
        .map(|update| {
            let mut valid = true;
            let mut appears: HashMap<i32, bool> = HashMap::new();
            let mut appeared_before: HashMap<i32, bool> = HashMap::new();
            let update: Vec<i32> = update.split(',')
                .map(|num| {
                    let num = num.parse::<i32>().unwrap();
                    *appears.entry(num).or_insert(true) = true;
                    num
                }).collect();

            for num in &update {
                *appeared_before.entry(*num).or_insert(true) = true;

                let requirements = rule_map.get(num).unwrap();
                let pass_requirements = requirements.iter()
                    .map(|required| {
                        !*appears.get(required).unwrap_or(&false) ||
                        *appeared_before.get(required).unwrap_or(&false)
                    })
                    .all(|b| b); // all true

                if !pass_requirements {
                    valid = false;
                }
            }

            if valid {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn part2() -> Result<(), io::Error>{
    let contents = read_to_string("input/day05.txt")?;

    todo!("obtain part2");

    let result = 0; // some calculation

    println!("result = {result}");
    
    Ok(())
}

pub fn answer() -> Result<(), io::Error>{
    println!("Part1:");
    part1()?;

    println!("Part2:");
    part2()?;

    Ok(())
}
