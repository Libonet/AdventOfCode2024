use std::{collections::HashMap, fs::read_to_string, io, ops::{Add, Sub}};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Pos (i32, i32);

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Self (self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self (self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    rows: Vec<Vec<char>>,
    row_count: usize,
    width: usize,
}

impl Matrix {
    fn new(rows: Vec<Vec<char>>) -> Self {
        assert!(!rows.is_empty());
        let width = rows[0].len();
        assert!(rows.iter().map(|row| row.len()).all(|len| len == width));

        let row_count = rows.len();
        Self { rows, row_count, width }
    }

    fn get(&self, pos: &Pos) -> Option<&char> {
        let Pos(x,y) = *pos;

        self.rows.get(x as usize)?.get(y as usize)
    }

    fn get_mut(&mut self, pos: &Pos) -> Option<&mut char> {
        let Pos(x,y) = *pos;

        self.rows.get_mut(x as usize)?.get_mut(y as usize)
    }
}

fn part1() -> Result<(), io::Error>{
    let contents = read_to_string("input/day08.txt")?; // input/dayxx.txt

    let rows: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let matrix = Matrix::new(rows);

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut char_positions = HashMap::with_capacity(alphabet.len());
    for chr in alphabet.chars() {
        char_positions.insert(chr, Vec::<Pos>::new());
    }

    let mut ret_matrix = matrix.clone();
    for x in 0..ret_matrix.row_count {
        for y in 0..ret_matrix.width {
            let pos = Pos(x as i32,y as i32);
            *ret_matrix.get_mut(&pos).unwrap() = '.';

            let chr = *matrix.get(&pos).unwrap();
            let vec_ref = char_positions.entry(chr).or_default();
            vec_ref.push(pos);
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
    for x in 0..ret_matrix.row_count {
        for y in 0..ret_matrix.width {
            let pos = Pos(x as i32,y as i32);
            if *ret_matrix.get(&pos).unwrap() == '$' {
                result += 1;
            }
        }
    }

    println!("result = {result}");

    Ok(())
}

fn part2() -> Result<(), io::Error>{
    let _contents = read_to_string("input/day08.txt")?;

    let _alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    todo!("obtain part2");

    let result = 0; // so rust shuts up
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
        
        let rows: Vec<Vec<char>> = contents.replace("\n", "").lines().map(|line| line.chars().collect::<Vec<char>>()).collect();

        let matrix = Matrix::new(rows);

        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut char_positions = HashMap::with_capacity(alphabet.len());
        for chr in alphabet.chars() {
            char_positions.insert(chr, Vec::<Pos>::new());
        }

        let mut ret_matrix = matrix.clone();
        for x in 0..ret_matrix.row_count {
            for y in 0..ret_matrix.width {
                let pos = Pos(x as i32,y as i32);
                *ret_matrix.get_mut(&pos).unwrap() = '.';

                let chr = *matrix.get(&pos).unwrap();
                let vec_ref = char_positions.entry(chr).or_default();
                vec_ref.push(pos);
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
        for x in 0..ret_matrix.row_count {
            for y in 0..ret_matrix.width {
                let pos = Pos(x as i32,y as i32);
                if *ret_matrix.get(&pos).unwrap() == '$' {
                    result += 1;
                }
            }
        }

        assert_eq!(result, 14);
    }
}
