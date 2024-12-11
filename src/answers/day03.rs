use std::{fs::read_to_string, io};
use crate::scanner::{Action, Scanner};

fn parse_mul(scanner: &mut Scanner) -> Option<i32> {
    let result = scanner.scan(|symbol| {
        match symbol {
            "m" => Some(Action::Require),
            "mu" => Some(Action::Require),
            "mul" => Some(Action::Require),
            "mul(" => Some(Action::Return(true)),
            _ => None,
        }
    });

    if let Ok(option) = result {
        if option.is_none() {
            scanner.pop();
            return None;
        }
    } else {
        return None;
    }

    let left = scanner.try_i32();
    if left.is_none() {
        scanner.pop();
        return None;
    }
    let left = left.unwrap();

    if left > 999 {
        return None;
    }

    if !scanner.take(&',') {
        return None
    }
    
    let right = scanner.try_i32();
    if right.is_none() {
        scanner.pop();
        return None;
    }
    let right = right.unwrap();

    if right > 999 {
        return None;
    }

    if !scanner.take(&')') {
        return None
    }

    Some(left * right)
}

fn part1(text: &str) -> i32 {
    let result: i32 = text.lines()
        .map(|line| {
            let mut scanner = Scanner::new(line);
            let mut sum = 0;
            while !scanner.is_done() {
                if let Some(num) = parse_mul(&mut scanner) {
                    sum += num;
                }
            }
            
            sum
        })
        .sum();

    println!("part1 result = {result}");

    result
}

fn parse_do(scanner: &mut Scanner) {
    if let Some(char) = scanner.peek() {
        if *char != 'd' {
            scanner.pop();
            return;
        }
    } else {
        return;
    }

    let result = scanner.scan(|symbol| {
        match symbol {
            "d" => Some(Action::Require),
            "do" => Some(Action::Require),
            "do(" => Some(Action::Require),
            "do()" => Some(Action::Return(true)),
            _ => None,
        }
    });

    if let Ok(option) = result {
        if option.is_some() {
            scanner.r#do();
        }
    }
}

fn parse_dont(scanner: &mut Scanner) -> bool {
    let result = scanner.scan(|symbol| {
        match symbol {
            "d" => Some(Action::Require),
            "do" => Some(Action::Require),
            "don" => Some(Action::Require),
            "don'" => Some(Action::Require),
            "don't" => Some(Action::Require),
            "don't(" => Some(Action::Require),
            "don't()" => Some(Action::Return(true)),
            _ => None,
        }
    });

    if let Ok(option) = result {
        if option.is_some() {
            scanner.dont();
            return true;
        } 
    }

    false
}

fn part2(text: &str) -> i32 {
    let mut scanner = Scanner::new(text);
    let mut result: i32 = 0;

    while !scanner.is_done() {
        if scanner.enabled() {
            if !parse_dont(&mut scanner) {
                if let Some(num) = parse_mul(&mut scanner) {
                    result += num;
                }
            }
        } else {
            parse_do(&mut scanner);
        }
    }

    println!("part2 result = {result}");

    result
}

pub fn answer() -> Result<(), io::Error> {
    let contents = read_to_string("input/day03.txt")?;

    let text = contents.lines().collect::<Vec<&str>>().join(", ");

    println!("Part1:");
    let _ = part1(&text);

    println!("Part2:");
    let _ = part2(&text);

    Ok(())
}

