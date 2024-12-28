use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, io};
use std::time::Instant;

type Input = String;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day22.txt")?; // input/dayxx.txt
    
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

type Secret = u64;
fn part1(input: &Input) -> Secret {
    let mut sum = 0;
    let mut cache: HashMap<Secret, Secret> = HashMap::new();
    
    for val in input.lines().map(|line| line.parse::<Secret>().expect("should be number")) {
        let mut next = val;
        for _i in 0..2000 {
            next = evolve_number(&mut cache, next);
        }
        sum += next;
    }

    sum
}

fn evolve_number(cache: &mut HashMap<Secret,Secret>, secret: Secret) -> Secret {
    if let Some(ret) = cache.get(&secret) {
        return *ret;
    }

    // first step:
    let next = secret << 6;
    let secret = mix(secret, next);
    let secret = prune(secret);

    // second step:
    let next = secret >> 5;
    let secret = mix(secret, next);
    let secret = prune(secret);

    // third step:
    let next = secret << 11;
    let secret = mix(secret, next);

    prune(secret)
}

fn mix(secret: Secret, val: Secret) -> Secret {
    secret ^ val
}

fn prune(secret: Secret) -> Secret {
    secret % 16777216
}

type Bananas = u64;
fn part2(input: &Input) -> Bananas {
    let mut cache: HashMap<Secret, Secret> = HashMap::new();
    
    let mut best_value: HashMap<(i16, i16, i16, i16), u64> = HashMap::new();
    for val in input.lines().map(|line| line.parse::<Secret>().expect("should be number")) {
        let mut found_changes: HashSet<(i16,i16,i16,i16)> = HashSet::new();
        let mut timeline = Vec::with_capacity(2000);
        let mut next = val;
        let mut prev_digit = get_digit(val);

        for _i in 0..2000 {
            next = evolve_number(&mut cache, next);
            let curr_digit = get_digit(next);
            let diff = curr_digit as i16 - prev_digit as i16;
            timeline.push((curr_digit, diff));
            prev_digit = curr_digit;
        }

        let (_v1,mut d1) = timeline[0];
        let (_v2,mut d2) = timeline[1];
        let (_v3,mut d3) = timeline[2];
        for (v4,d4) in timeline.into_iter().skip(3) {
            if !found_changes.contains(&(d1,d2,d3,d4)) {
                found_changes.insert((d1,d2,d3,d4));
                best_value.entry((d1,d2,d3,d4)).and_modify(|v| *v += v4 as Bananas).or_default();
            }

            d1 = d2;
            d2 = d3;
            d3 = d4;
        }
    }

    let mut max: Bananas = 0;
    for d1 in -9..=9 {
        for d2 in -9..=9 {
            for d3 in -9..=9 {
                for d4 in -9..=9 {
                    if let Some(val) = best_value.get(&(d1,d2,d3,d4)) {
                        if *val > max {
                            max = *val;
                        }
                    }
                }
            }
        }
    }

    max
}

fn get_digit(secret: Secret) -> u8 {
    (secret % 10) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let contents = "\
1
10
100
2024".to_string();

        let input = parse(contents);

        let result = part1(&input);

        assert_eq!(result, 37327623);
    }
}
