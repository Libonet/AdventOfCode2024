use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, io}; use std::time::Instant;

type Memo = HashMap<usize, usize>;
type Input = (HashSet<String>, Vec<String>, usize);
type Count = usize;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day19.txt")?; // input/dayxx.txt
    
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
    let (list, targets) = contents.split_once("\n\n").unwrap();

    let mut max_towel_size = 0;
    let towels = list
        .split(", ")
        .map(|pat| {
            let ret = pat.to_string();
            let len = ret.len();
            if len > max_towel_size {
                max_towel_size = len;
            }

            ret
        })
        .collect();

    let targets = targets
        .lines()
        .map(|line| line.to_string())
        .collect();

    (towels, targets, max_towel_size)
}

fn part1(input: &Input) -> Count {
    let (towels, targets, max_towel_size) = input;

    let mut count = 0;
    for target in targets {
        if can_build(target, towels, max_towel_size) {
            count += 1;
        }
    }

    count
}

fn can_build(target: &str, towels: &HashSet<String>, max_towel_size: &usize) -> bool {
    let mut start_index: Vec<usize> = vec![0];

    let mut built: HashSet<usize> = HashSet::new();

    while let Some(pos) = start_index.pop() {
        if !built.insert(pos) {
            // if already checked, skip
            continue;
        }

        // make every sub_string from the target's starting position
        let sub_targets = (0..*max_towel_size)
            .filter_map(|step| {
                if step + pos < target.len() {
                    Some(target[pos..=pos+step].to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for sub_target in sub_targets {
            if towels.contains(&sub_target) {
                let len = pos + sub_target.len();

                // if the sub_target reaches the end from 
                // it's start position, we found a match
                if len == target.len() {
                    return true;
                } else {
                    start_index.push(len);
                }
            }
        }
    }

    false
}

fn part2(input: &Input) -> Count {
    let (towels, targets, max_towel_size) = input;


    let mut count = 0;
    for target in targets {
        count += 
        number_of_ways(
            target,
            0,
            towels,
            max_towel_size,
            &mut Memo::new());
    }

    count
}


fn number_of_ways(
    target: &str,
    pos: usize,
    towels: &HashSet<String>,
    max_towel_size: &usize,
    memo: &mut Memo,
) -> Count {
    if let Some(val) = memo.get(&pos) {
        return *val;
    }

    let mut count = 0;
    // make every sub_string from the target's starting position
    let sub_targets = (0..*max_towel_size)
        .filter_map(|step| {
            let offset = step + pos;
            if offset < target.len() {
                Some(target[pos..=offset].to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for sub_target in sub_targets {
        if towels.contains(&sub_target) {
            let len = pos + sub_target.len();

            // if the sub_target reaches the end from 
            // it's start position, we found a match
            if len == target.len() {
                count += 1;
            } else {
                count += number_of_ways(
                    target,
                    len,
                    towels,
                    max_towel_size,
                    memo
                );
            }
        }
    }

    memo.insert(pos, count);

    count as Count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_ways() {
        let input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb".to_string();

        let input = parse(input);

        let result = part2(&input);

        assert_eq!(result, 16);
    }
}
