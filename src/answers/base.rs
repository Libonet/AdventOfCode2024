use std::{fs::read_to_string, io};

type Input = String;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("")?; // input/dayxx.txt
    
    let input = parse(contents);
    
    println!("Part1:");
    let part1_res = part1(&input);
    println!("result = {part1_res}");

    println!("Part2:");
    let part2_res = part2(&input);
    println!("result = {part2_res}");

    Ok(())
}

fn part1(contents: &Input) -> i32 {
    let result = 0; // so rust shuts up

    result
}

fn part2(contents: &Input) -> i32 {
    let result = 0; // so rust shuts up

    result
}

fn parse(contents: String) -> Input {
    contents
}
