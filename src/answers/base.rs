use std::{fs::read_to_string, io};

fn part1(contents: String){
    todo!("obtain part1");

    let result = 0; // so rust shuts up
    println!("result = {result}");
}

fn part2(contents: String){
    todo!("obtain part2");

    let result = 0; // so rust shuts up
    println!("result = {result}");
}

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("")?; // input/dayxx.txt
    
    println!("Part1:");
    let _ = part1(contents.clone());

    println!("Part2:");
    let _ = part2(contents);

    Ok(())
}
