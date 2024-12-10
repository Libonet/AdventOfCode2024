use std::{collections::HashMap, fs::read_to_string, io};
use crate::matrix::{Matrix, Pos};

fn part1() -> Result<(), io::Error>{
    let contents = read_to_string("input/day08.txt")?; // input/dayxx.txt

    let width = contents.find("\n").unwrap();
    let rows: Vec<char> = contents.replace("\n", "").chars().collect();

    let matrix = Matrix::new(rows, width);

    let result = find_antinodes(matrix);

    println!("result = {result}");

    Ok(())
}

fn find_antinodes(matrix: Matrix<char>) -> i32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut char_positions = HashMap::with_capacity(alphabet.len());
    for chr in alphabet.chars() {
        char_positions.insert(chr, Vec::<Pos>::new());
    }

    let mut ret_matrix = Matrix::with_capacity(matrix.row_count(), matrix.width()); 

    let mut pos = Pos(0, 0);
    for mat_entry in &matrix {
        ret_matrix.push('.');

        let vec_ref = char_positions.entry(*mat_entry).or_default();
        vec_ref.push(pos);
        pos.1 += 1;
        if pos.1 >= matrix.width() as i32 {
            pos.0 += 1;
            pos.1 = 0;
        }
    }

    for chr in alphabet.chars() {
        let positions = char_positions.get(&chr).unwrap();

        for i in 0..positions.len() {
            let start = positions[i];

            for end in positions.iter().skip(i+1) {
                let offset = *end - start;

                if let Some(entry) = ret_matrix.get_mut(&(start-offset)) {
                    *entry = '$';
                }
                if let Some(entry) = ret_matrix.get_mut(&(*end+offset)) {
                    *entry = '$';
                }
            }
        }
    }

    let mut result = 0;
    for val in ret_matrix {
        if val == '$' {
            result += 1;
        }
    }

    result
}

fn part2() -> Result<(), io::Error>{
    let contents = read_to_string("input/day08.txt")?; // input/dayxx.txt

    let width = contents.find("\n").unwrap();
    let rows: Vec<char> = contents.replace("\n", "").chars().collect();

    let matrix = Matrix::new(rows, width);

    let result = find_antinodes_harmonics(matrix);
    
    println!("result = {result}");

    Ok(())
}

fn find_antinodes_harmonics(matrix: Matrix<char>) -> i32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut char_positions = HashMap::with_capacity(alphabet.len());
    for chr in alphabet.chars() {
        char_positions.insert(chr, Vec::<Pos>::new());
    }

    let mut ret_matrix = Matrix::with_capacity(matrix.row_count(), matrix.width());

    let mut pos = Pos(0, 0);
    for mat_entry in &matrix {
        ret_matrix.push('.');

        let vec_ref = char_positions.entry(*mat_entry).or_default();
        vec_ref.push(pos);
        pos.1 += 1;
        if pos.1 >= matrix.width() as i32 {
            pos.0 += 1;
            pos.1 = 0;
        }
    }

    for chr in alphabet.chars() {
        let positions = char_positions.get(&chr).unwrap();

        if positions.len() > 1 {
            for i in 0..positions.len() {
                let start = positions[i];

                for end in positions.iter().skip(i+1) {
                    let offset = *end - start;

                    let mut times = 0;
                    while let Some(entry) = ret_matrix.get_mut(&(start+(offset*times))) {
                        *entry = '$';
                        times += 1;
                    }
                    let mut times = 1;
                    while let Some(entry) = ret_matrix.get_mut(&(start-(offset*times))) {
                        *entry = '$';
                        times += 1;
                    }
                }
            }
        }
    }

    let mut result = 0;
    for val in ret_matrix {
        if val == '$' {
            result += 1;
        }
    }
    
    result
}

pub fn answer() -> Result<(), io::Error>{
    println!("Part1:");
    part1()?;

    println!("Part2:");
    part2()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let contents = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        
        let width = contents.find("\n").unwrap();
        let rows: Vec<char> = contents.replace("\n", "").chars().collect();

        let matrix = Matrix::new(rows, width);

        let result = find_antinodes(matrix);
        
        assert_eq!(result, 14);
    }

    #[test]
    fn part2_example() {
        let contents = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        
        let width = contents.find("\n").unwrap();
        let rows: Vec<char> = contents.replace("\n", "").chars().collect();

        let matrix = Matrix::new(rows, width);

        let result = find_antinodes_harmonics(matrix);
        
        assert_eq!(result, 34);
    }

    #[test]
    fn part2_example2() {
        let contents = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

        let width = contents.find("\n").unwrap();
        let rows: Vec<char> = contents.replace("\n", "").chars().collect();

        let matrix = Matrix::new(rows, width);

        let result = find_antinodes_harmonics(matrix);
        
        assert_eq!(result, 9);
    }
}
