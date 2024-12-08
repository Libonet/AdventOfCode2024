use std::{collections::HashMap, fs::read_to_string, io, ops::{Add, Sub, Mul}};

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

impl Mul<i32> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i32) -> Self::Output {
        Self (self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    rows: Vec<Vec<char>>,
    row_count: usize,
    width: usize,
    pos: Pos,
}

impl Matrix {
    fn new(rows: Vec<Vec<char>>) -> Self {
        assert!(!rows.is_empty());
        let width = rows[0].len();
        assert!(rows.iter().map(|row| row.len()).all(|len| len == width));

        let row_count = rows.len();
        let pos = Pos(0,0);
        Self { rows, row_count, width, pos }
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

impl Iterator for Matrix {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.1 < self.width as i32 {
            let pos = self.pos;
            self.pos.1 += 1;

            self.get(&pos).copied()
        } else if self.pos.0 < self.row_count as i32 {
            let pos = self.pos;
            self.pos.0 += 1;
            self.pos.1 = 0;

            self.get(&pos).copied()
        } else {
            None
        }
    }
}

fn part1() -> Result<(), io::Error>{
    let contents = read_to_string("input/day08.txt")?; // input/dayxx.txt

    let rows: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let matrix = Matrix::new(rows);

    let result = find_antinodes(matrix);

    println!("result = {result}");

    Ok(())
}

fn find_antinodes(matrix: Matrix) -> i32 {
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

    result
}

fn part2() -> Result<(), io::Error>{
    let contents = read_to_string("input/day08.txt")?; // input/dayxx.txt

    let rows: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let matrix = Matrix::new(rows);

    let result = find_antinodes_harmonics(matrix);
    
    println!("result = {result}");

    Ok(())
}

fn find_antinodes_harmonics(matrix: Matrix) -> i32 {
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
    for x in 0..ret_matrix.row_count {
        for y in 0..ret_matrix.width {
            let pos = Pos(x as i32,y as i32);
            if *ret_matrix.get(&pos).unwrap() == '$' {
                result += 1;
            }
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
        
        let rows: Vec<Vec<char>> = contents.replace("\n", "").lines().map(|line| line.chars().collect::<Vec<char>>()).collect();

        let matrix = Matrix::new(rows);

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
        
        let rows: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let matrix = Matrix::new(rows);

        let result = find_antinodes_harmonics(matrix);
        
        assert_eq!(result, 34);
    }

    #[test]
    fn part2_example2() {
        let contents = "\
T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";

        let rows: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let matrix = Matrix::new(rows);

        let result = find_antinodes_harmonics(matrix);
        
        assert_eq!(result, 9);
    }
}
