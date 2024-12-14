use std::{io, process::exit};
use advent_of_code2024::answers;

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

    match num {
        1 => get_answer(1, answers::day01::answer),
        2 => get_answer(2, answers::day02::answer),
        3 => get_answer(3, answers::day03::answer),
        4 => get_answer(4, answers::day04::answer),
        5 => get_answer(5, answers::day05::answer),
        6 => get_answer(6, answers::day06::answer),
        7 => get_answer(7, answers::day07::answer),
        8 => get_answer(8, answers::day08::answer),
        9 => get_answer(9, answers::day09::answer),
        10 => get_answer(10, answers::day10::answer),
        11 => get_answer(11, answers::day11::answer),
        12 => get_answer(12, answers::day12::answer),
        13 => get_answer(13, answers::day13::answer),
        14 => get_answer(14, answers::day14::answer),
        _ => {
            eprintln!("Error: Day should exist");
            exit(2);
        }
    }
}

fn get_answer(day: i32, answer: impl Fn() -> Result<(), io::Error>) {
    if let Err(e) = answer() {
        eprintln!("Error on Day {day}: {e}");
        exit(3);
    }
}

