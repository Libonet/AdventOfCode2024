use std::{collections::HashMap, fs::read_to_string, io};

fn part1() -> Result<(), io::Error>{
    let contents = read_to_string("input/day01.txt")?;

    let (mut nums1, mut nums2): (Vec<i32>, Vec<i32>) = contents.lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (iter.next().unwrap().parse::<i32>().unwrap(),
             iter.next().unwrap().parse::<i32>().unwrap())
        })
        .collect();

    nums1.sort();
    nums2.sort();

    let total_distance: u32 = nums1.iter().zip(nums2.iter())
        .map(|(a,b)| a.abs_diff(*b))
        .sum();

    println!("total distance = {total_distance}");

    Ok(())
}

fn part2() -> Result<(), io::Error>{
    let contents = read_to_string("input/day01.txt")?;

    let (nums1, nums2): (Vec<i32>, Vec<i32>) = contents.lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (iter.next().unwrap().parse::<i32>().unwrap(),
             iter.next().unwrap().parse::<i32>().unwrap())
        })
        .collect();

    let mut map: HashMap<i32,i32> = HashMap::with_capacity(nums2.len());

    for num in nums2.iter() {
        map.entry(*num).and_modify(|count| *count += 1).or_insert(1);
    }

    let similarity_score: i32 = nums1.iter()
        .map(|num| {
            match map.get(num) {
                Some(n) => *n * *num,
                None => 0,
            }
        })
        .sum();

    println!("similarity score = {similarity_score}");

    Ok(())
}

pub fn answer() -> Result<(), io::Error>{
    println!("Part1:");
    part1()?;

    println!("Part2:");
    part2()?;

    Ok(())
}
