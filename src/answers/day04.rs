use std::{fs::read_to_string, io};

// Thanks to capito27 for the better way of masking the input

struct Mask {
    relative_pos: Vec<isize>,
    left_offset: isize,
    right_offset: isize,
}

impl Mask {
    fn new(relative_pos: Vec<isize>, left_offset: isize, right_offset: isize) -> Self {
        Self {
            relative_pos,
            left_offset,
            right_offset,
        }
    }

    fn apply<'a, T>(&self, pos: isize, width: isize, vals: &'a [T]) -> Option<Vec<&'a T>> {
        if pos % width < self.left_offset || (pos % width) + self.right_offset >= width {
            return None;
        }

        self.relative_pos
            .iter()
            .map(|&i| vals.get((i+pos) as usize))
            .collect::<Option<Vec<_>>>()
    }
}

/// Because the input is one string, we can move between rows by adding
/// the width to the relative position.
fn xmas_masks(w: isize) -> Vec<Mask> {
    vec![
        // ->
        Mask::new(vec![0, 1, 2, 3], 0, 3),
        // diag_right_down
        Mask::new(vec![0, w+1, 2*w+2, 3*w+3], 0, 3),
        // down
        Mask::new(vec![0, w, 2*w, 3*w], 0, 0),
        // diag_left_down
        Mask::new(vec![0, w-1, 2*w-2, 3*w-3], 3, 0),
        // <-
        Mask::new(vec![0, -1, -2, -3], 3, 0),
        // diag_left_up
        Mask::new(vec![0, -w-1, -2*w-2, -3*w-3], 3, 0),
        // up
        Mask::new(vec![0, -w, -2*w, -3*w], 0, 0),
        // diag_right_up
        Mask::new(vec![0, -w+1, -2*w+2, -3*w+3], 0, 3),
    ]
}

fn part1() -> Result<(), io::Error> {
    let contents = read_to_string("input/day04.txt")?;

    let width = contents.find("\n").unwrap() as isize;
    let contents = contents.replace("\n", "").chars().collect::<Vec<char>>();
    let contents_slice = contents.as_slice();

    let masks = xmas_masks(width);

    let result = (0..contents.len() as isize)
        .flat_map(|pos| {
            masks
                .iter()
                .filter_map(move |mask| mask.apply(pos, width, contents_slice))
        })
        .filter(|vals| {
            vals.len() == 4 && parse_xmas(vals)
        })
        .count() as u32;

    println!("part1 result = {result}");

    Ok(())
}

fn parse_xmas(text: &[&char]) -> bool {
    *text[0] == 'X' && *text[1] == 'M' && *text[2] == 'A' && *text[3] == 'S'
}

/// Valid masks should equal MSAMS
fn x_mas_masks(w: isize) -> Vec<Mask>{
    vec![
        // M.S
        // .A.
        // M.S
        Mask::new(vec![0, 2, w+1, 2*w, 2*w+2], 0, 2),
        // S.S
        // .A.
        // M.M
        Mask::new(vec![2*w, 0, w+1, 2*w+2, 2], 0, 2),
        // M.M
        // .A.
        // S.S
        Mask::new(vec![0, 2*w, w+1, 2, 2*w+2], 0, 2),
        // S.M
        // .A.
        // S.M
        Mask::new(vec![2, 0, w+1, 2*w+2, 2*w], 0, 2),
    ]
}

fn parse_x_mas(text: &[&char]) -> bool {
    *text[0] == 'M' && *text[1] == 'S' && *text[2] == 'A' && *text[3] == 'M' && *text[4] == 'S'
}

fn part2() -> Result<(), io::Error> {
    let contents = read_to_string("input/day04.txt")?;

    let width = contents.find("\n").unwrap() as isize;
    let contents = contents.replace("\n", "").chars().collect::<Vec<char>>();
    let contents_slice = contents.as_slice();

    let masks = x_mas_masks(width);

    let result = (0..contents.len() as isize)
        .flat_map(|pos| {
            masks
                .iter()
                .filter_map(move |mask| mask.apply(pos, width, contents_slice))
        })
        .filter(|vals| {
            vals.len() == 5 && parse_x_mas(vals)
        })
        .count() as u32;

    println!("part1 result = {result}");

    Ok(())
}

pub fn answer() -> Result<(), io::Error> {
    println!("Part1:");
    part1()?;

    println!("Part2:");
    part2()?;

    Ok(())
}
