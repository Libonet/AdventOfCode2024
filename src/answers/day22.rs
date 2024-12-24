use std::collections::HashMap;
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
