use std::{collections::{HashSet, VecDeque}, fs::read_to_string, io, ops::Add};
use crate::{
    matrix::{Matrix, Mask},
    position::Pos,
};

fn part1(contents: &str) -> i32 {
    let width = contents.find("\n").unwrap();
    let rows = contents.replace("\n", "").chars().collect();

    let matrix = Matrix::new(rows, width);

    let mut trailheads: HashSet<Pos> = HashSet::new();

    for (char, pos) in matrix.give_pos() {
        if *char == '0' {
            trailheads.insert(pos);
        }
    }

    let result = trailheads.iter()
        .map(|pos| {
            find_score(&matrix, *pos)
        })
        .sum();

    println!("result = {result}");

    result
}

fn find_score(matrix: &Matrix<char>, pos: Pos) -> i32 {
    let masks = create_masks();

    let mut trailends: HashSet<Pos> = HashSet::new();
    let mut candidates: VecDeque<Pos> = VecDeque::new();
    candidates.push_back(pos);

    while !candidates.is_empty() {
        let pos = candidates.pop_front().unwrap();
        let num_pos = matrix.get_pos(&pos).unwrap().to_digit(10).unwrap();

        for (i, mask) in masks.iter().enumerate() {
            let values = mask.apply(pos, matrix);
            if let Some(Some(val)) = values.first() {
                let num_val = (**val).to_digit(10).unwrap();

                if num_val > num_pos && num_val.abs_diff(num_pos) == 1 {
                    let val_pos = pos + Dir::match_num(i);
                    if num_val == 9 {
                        trailends.insert(val_pos);
                    } else {
                        candidates.push_back(val_pos);
                    }
                }
            }
        }
    }

    trailends.len() as i32
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn match_num(num: usize) -> Dir {
        assert!((0..=3).contains(&num));
        match num {
            0 => Dir::Up,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Right,
            _ => unreachable!("num restricted to range 0..=3"),
        }
    }
}

impl Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::Up => self + Pos(-1,0),
            Dir::Down => self + Pos(1,0),
            Dir::Left => self + Pos(0,-1),
            Dir::Right => self + Pos(0,1),
        }
    }
}

fn create_masks() -> Vec<Mask> {
    vec![
        // up
        Mask::new(vec![Pos(-1, 0)]),
        // down
        Mask::new(vec![Pos(1, 0)]),
        // left
        Mask::new(vec![Pos(0, -1)]),
        // right
        Mask::new(vec![Pos(0, 1)]),
    ]
}

fn part2(contents: &str) -> i32 {
    let width = contents.find("\n").unwrap();
    let rows = contents.replace("\n", "").chars().collect();

    let matrix = Matrix::new(rows, width);

    let mut trailheads: HashSet<Pos> = HashSet::new();

    for (char, pos) in matrix.give_pos() {
        if *char == '0' {
            trailheads.insert(pos);
        }
    }

    let result = trailheads.iter()
        .map(|pos| {
            find_rating(&matrix, *pos)
        })
        .sum();

    println!("result = {result}");

    result
}

fn find_rating(matrix: &Matrix<char>, pos: Pos) -> i32 {
    let masks = create_masks();

    let mut trailends: Vec<Pos> = Vec::new();
    let mut candidates: VecDeque<Pos> = VecDeque::new();
    candidates.push_back(pos);

    while !candidates.is_empty() {
        let pos = candidates.pop_front().unwrap();
        let num_pos = matrix.get_pos(&pos).unwrap().to_digit(10).unwrap();

        for (i, mask) in masks.iter().enumerate() {
            let values = mask.apply(pos, matrix);
            if let Some(Some(val)) = values.first() {
                let num_val = (**val).to_digit(10).unwrap();

                if num_val > num_pos && num_val.abs_diff(num_pos) == 1 {
                    let val_pos = pos + Dir::match_num(i);
                    if num_val == 9 {
                        trailends.push(val_pos);
                    } else {
                        candidates.push_back(val_pos);
                    }
                }
            }
        }
    }

    trailends.len() as i32
}

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day10.txt")?; // input/dayxx.txt
    
    println!("Part1:");
    let _ = part1(&contents);

    println!("Part2:");
    let _ = part2(&contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let contents = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let result = part1(contents);

        assert_eq!(result, 36);
    }
}
