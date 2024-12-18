use std::{fs::read_to_string, io};
use std::time::Instant;

type Input = String;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("")?; // input/dayxx.txt
    
    let input = parse(contents);
    
    println!("Part1:");
    let now = Instant::now();
    let part1_res = part1(&input);
    let elapsed = now.elapsed();
    println!("result = {part1_res}");
    println!("Time taken: {:.2?}", elapsed);

    println!("Part2:");
    let now = Instant::now();
    let part2_res = part2(&input);
    let elapsed = now.elapsed();
    println!("result = {part2_res:?}");
    println!("Time taken: {:.2?}", elapsed);

    Ok(())
}

fn parse(contents: String) -> Input {
    contents
}

fn part1(input: &Input) -> i32 {
    let result = 0; // so rust shuts up

    result
}

fn part2(input: &Input) -> i32 {
    let result = 0; // so rust shuts up

    result
}
