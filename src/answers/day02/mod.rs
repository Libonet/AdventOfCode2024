use std::{fs::read_to_string, io};

fn increases(level: &[i32]) -> bool {
    let mut level = level.iter();
    let mut current = *level.next().expect("Should not be empty");

    for num in level {
        let diff = *num - current;

        if !(1..=3).contains(&diff) {
            return false;
        }

        current = *num;
    }

    true
}

fn decreases(level: &[i32]) -> bool {
    let mut level = level.iter();
    let mut current = *level.next().expect("Should not be empty");

    for num in level {
        let diff = current - *num;

        if !(1..=3).contains(&diff) {
            return false;
        }
        
        current = *num;
    }

    true
}

fn part1() -> Result<(), io::Error>{
    let contents = read_to_string("input/day02.txt")?;

    let reports = contents.lines();

    let safe_reports = reports
        .map(|line| 
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Should be number"))
                .collect::<Vec<i32>>())
        .filter(|level| increases(level) || decreases(level))
        .count();
        
    println!("safe reports = {safe_reports}");

    Ok(())
}

fn increases_dampened(level: &[i32]) -> bool {
    let mut current = *level.first().expect("Should not be empty");
    let mut previous: Option<i32> = None;

    for i in 1..level.len() {
        let num = level[i];
        let diff = num - current;

        if !(1..=3).contains(&diff) {
            if let Some(n) = previous {
                let new_level = [&[n], &level[i..]].concat();
                if increases(&new_level) {
                    return true;
                }
            } else if increases(&level[i..]){
                return true;
            }
            let new_level = [&[current], &level[i+1..]].concat();
            return increases(&new_level);
        } else {
            previous = Some(current);
            current = num;
        }
    }

    true
}

fn decreases_dampened(level: &[i32]) -> bool {
    let mut current = *level.first().expect("Should not be empty");
    let mut previous: Option<i32> = None;

    for i in 1..level.len() {
        let num = level[i];
        let diff = current - num;

        if !(1..=3).contains(&diff) {
            if let Some(n) = previous {
                let new_level = [&[n], &level[i..]].concat();
                if decreases(&new_level) {
                    return true;
                }
            } else if decreases(&level[i..]) {
                return true;
            }
            let new_level = [&[current], &level[i+1..]].concat();
            return decreases(&new_level);
        } else {
            previous = Some(current);
            current = num;
        }
    }

    true
}

fn part2() -> Result<(), io::Error>{
    let contents = read_to_string("input/day02.txt")?;

    let reports = contents.lines();

    let safe_reports = reports
        .map(|line| 
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Should be number"))
                .collect::<Vec<i32>>())
        .filter(|level| increases_dampened(level) || decreases_dampened(level))
        .count();
        
    println!("safe reports = {safe_reports}");

    Ok(())
}

pub fn answer() -> Result<(), io::Error>{
    println!("Part1:");
    part1()?;

    println!("Part2:");
    part2()?;

    Ok(())
}

