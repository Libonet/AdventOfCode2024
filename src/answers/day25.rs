use std::{fs::read_to_string, io};
use std::time::Instant;

type Key = [u8; 5];
type Lock = [u8; 5];

type Keys = Vec<Key>;
type Locks = Vec<Lock>;
type Input = (Keys, Locks);

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day25.txt")?; // input/dayxx.txt
    
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
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for schematic in contents.split("\n\n") {
        let mut lines = schematic.lines();
        let first = lines.next().unwrap();

        let mut schem_vals = [0; 5];
        for line in lines.take(5) {
            for (i,char) in line.chars().enumerate() {
                if char == '#' {
                    schem_vals[i] += 1;
                }
            }
        }

        if first == "#####" {
            keys.push(schem_vals);
        } else {
            locks.push(schem_vals);
        }
    }

    (keys, locks)
}

type Count = u32;
fn part1(input: &Input) -> Count {
    let (keys, locks) = input;

    let mut count = 0;
    for lock in locks {
        for key in keys {
            if key_fits(key, lock) {
                count += 1;
            }
        }
    }

    count
}

fn key_fits(key: &Key, lock: &Lock) -> bool {
    for i in 0..key.len() {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }

    true
}

fn part2(input: &Input) -> String {
    "FINISHED AOC 2024! :D".to_string()
}
