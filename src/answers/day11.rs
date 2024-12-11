use std::{collections::HashMap, fs::read_to_string, io};

type Num = u64;
type Input = HashMap<Num, usize>;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day11.txt")?; // input/dayxx.txt
    
    println!("Part1:");
    let _ = part1(&contents);

    println!("Part2:");
    let _ = part2(&contents);

    Ok(())
}

fn part1(contents: &str) -> usize {
    let result = solve(contents, 25);

    println!("result = {result}");

    result
}

fn part2(contents: &str) -> usize {
    let result = solve(contents, 75);

    println!("result = {result}");

    result
}

fn solve(contents: &str, times: usize) -> usize {
    let contents = contents.split_whitespace();

    let mut stones = Input::default();
    for val in contents {
        let num: Num = val.parse().unwrap();
        *stones.entry(num).or_default() += 1;
    };

    for _i in 0..times {
        follow_rules(&mut stones);
    }

    stones.values().sum()
}

fn follow_rules(stones: &mut Input) {
    let old_stones: Vec<(u64, usize)> = stones.drain().collect();
    for (stone, count) in old_stones {
        if stone == 0 {
            add_to_stone(stones, 1, count);
        } else if even_digits(stone) {
            let (left, right) = split_stone(stone);
            add_to_stone(stones, left, count);
            add_to_stone(stones, right, count);
        } else {
            add_to_stone(stones, stone * 2024, count);
        }
    }
}

fn add_to_stone(stones: &mut Input, stone: Num, count: usize) {
    *stones.entry(stone).or_default() += count;
}

fn even_digits(stone: Num) -> bool {
    num_digits(stone) % 2 == 0
}

fn num_digits(stone: Num) -> u32 {
    stone.ilog10() + 1
}

fn split_stone(stone: Num) -> (Num, Num) {
    let half = num_digits(stone) / 2;
    let tens = 10u64.pow(half);

    let left = stone / tens;
    let right = stone - left*tens;

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";

        let result = part1(input);

        assert_eq!(result, 55312);
    }

    #[test]
    fn follow_rules_twice() {
        let contents = "125 17";
        let mut stones = Input::new();
        let contents = contents.split_whitespace();

        for val in contents {
            let val: Num = val.parse().unwrap();
            *stones.entry(val).or_default() += 1;
        };

        for _i in 0..2 {
            follow_rules(&mut stones);
        }
        
        let result: usize = stones.values().sum();

        assert_eq!(result, 4);
    }

    #[test]
    fn split_example() {
        let example = 123456;

        let res = split_stone(example);

        assert_eq!(res, (123, 456));
    }

    #[test]
    fn split_leading_zero() {
        let example = 12300456;

        let res = split_stone(example);

        assert_eq!(res, (1230, 456));
    }

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(12345), 5);
    }
}
