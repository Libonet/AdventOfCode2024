use std::{io, process::exit};

mod answers;
mod matrix;

fn main() {
    println!("Input the day to get the day's answer.");

    let stdin = io::stdin();

    let mut input = String::new();

    stdin.read_line(&mut input).expect("Should get a correct string");

    let num: i32 = match input.trim_end().parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    if !(1..=31).contains(&num) {
        eprintln!("Error: Day should exist");
        exit(2);
    }

    match num {
        1 => if let Err(e) = answers::day01::answer() {
            eprintln!("Error on Day 1: {e}");
            exit(3);
        },
        2 => if let Err(e) = answers::day02::answer() {
            eprintln!("Error on Day 2: {e}");
            exit(3);
        },
        3 => if let Err(e) = answers::day03::answer() {
            eprintln!("Error on Day 3: {e}");
            exit(3);
        },
        4 => if let Err(e) = answers::day04::answer() {
            eprintln!("Error on Day 4: {e}");
            exit(3);
        },
        5 => if let Err(e) = answers::day05::answer() {
            eprintln!("Error on Day 5: {e}");
            exit(3);
        },
        6 => if let Err(e) = answers::day06::answer() {
            eprintln!("Error on Day 6: {e}");
            exit(3);
        },
        7 => if let Err(e) = answers::day07::answer() {
            eprintln!("Error on Day 7: {e}");
            exit(3);
        },
        8 => if let Err(e) = answers::day08::answer() {
            eprintln!("Error on Day 8: {e}");
            exit(3);
        },
        9 => if let Err(e) = answers::day09::answer() {
            eprintln!("Error on Day 9: {e}");
            exit(3);
        },
        10 => if let Err(e) = answers::day10::answer() {
            eprintln!("Error on Day 10: {e}");
            exit(3);
        },
        _ => {
            eprintln!("Error: Day should exist");
            exit(2);
        }
    }
}
