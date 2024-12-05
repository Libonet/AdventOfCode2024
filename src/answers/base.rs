use std::{fs::read_to_string, io};

fn part1() -> Result<(), io::Error>{
    let contents = read_to_string("input/day01.txt")?;

    todo!("obtain part1");

    println!("result = {result}");

    Ok(())
}

fn part2() -> Result<(), io::Error>{
    let contents = read_to_string("input/day01.txt")?;

    todo!("obtain part2");

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
