use std::{fs::read_to_string, io};

fn part1(contents: String) -> i32 {
    let result = 0; // so rust shuts up

    result
}

fn part2(contents: String) -> i32 {
    let result = 0; // so rust shuts up

    result
}

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("")?; // input/dayxx.txt
    
    println!("Part1:");
    let part1_res = part1(contents.clone());
    println!("result = {part1_res}");

    println!("Part2:");
    let part2_res = part2(contents);
    println!("result = {part2_res}");

    Ok(())
}

