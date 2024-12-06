use std::{fs::read_to_string, io, collections::HashSet};

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

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Dirs {
    Up,
    Down,
    Left,
    Right,
}

impl Dirs {
    fn rotate(&mut self) {
        match self {
            Dirs::Right => *self = Dirs::Down,
            Dirs::Left => *self = Dirs::Up,
            Dirs::Down => *self = Dirs::Left,
            Dirs::Up => *self = Dirs::Right,
        }
    }
}

/// Because the input is one string, we can move between rows by adding
/// the width to the relative position.
fn direction_masks(w: isize) -> Vec<Mask> {
    // ...
    // .^.
    // ...
    vec![
        Mask::new(vec![-w], 0, 0),
        Mask::new(vec![w], 0, 0),
        Mask::new(vec![-1], 1, 0),
        Mask::new(vec![1], 0, 1),
    ]
}

fn part1() -> Result<(), io::Error>{
    let contents = read_to_string("input/day06.txt")?;

    let width = contents.find("\n").unwrap() as isize;
    let mut contents = contents.replace("\n", "").chars().collect::<Vec<char>>();

    let pos = find_guard(&contents);

    let masks = direction_masks(width);
    let mut initial_dir = Dirs::Up;

    let result = simulate_path(pos, width, &mut contents, &mut initial_dir, &masks);
    println!("result = {result}");

    Ok(())
}

fn find_guard(content: &[char]) -> isize {
    let mut pos = 0;
    for char in content {
        if *char == '^' {
            break;
        }
        pos += 1;
    }
    pos
}

fn get_mask<'a>(masks: &'a [Mask], dir: &Dirs) -> &'a Mask {
    match dir {
        Dirs::Up => &masks[0],
        Dirs::Down => &masks[1],
        Dirs::Left => &masks[2],
        Dirs::Right => &masks[3],
    }
}

fn simulate_path(pos: isize, w: isize, content: &mut [char], dir: &mut Dirs, masks: &[Mask]) -> i32 {
    let mut pos = pos;
    let mut res = 0;

    let mut mask = get_mask(masks, dir);

    loop {
        let curr_pos = content[pos as usize];
        if curr_pos != 'X' {
            res += 1;
            content[pos as usize] = 'X';
        }
        let look_ahead = mask.apply(pos, w, content);
        if let Some(char) = look_ahead {
            match char.first().unwrap() {
                '#' => { 
                    dir.rotate();
                    mask = get_mask(masks, dir);
                },
                _ => { pos = advance(pos, dir, w); },
            }
        } else {
            break;
        }
    }

    res
}

fn advance(pos: isize, dir: &Dirs, w: isize) -> isize {
    match dir {
        Dirs::Up => pos-w,
        Dirs::Down => pos+w,
        Dirs::Left => pos-1,
        Dirs::Right => pos+1,
    }
}

fn part2() -> Result<(), io::Error>{
    let contents = read_to_string("input/day06.txt")?;

    let width = contents.find("\n").unwrap() as isize;
    let mut contents = contents.replace("\n", "").chars().collect::<Vec<char>>();

    let initial_pos = find_guard(&contents);

    let masks = direction_masks(width);
    let mut initial_dir = Dirs::Up;

    let mut path_result = contents.clone();
    let _result = simulate_path(initial_pos, width, &mut path_result, &mut initial_dir, &masks);

    let mut result = 0;
    
    for pos in 0..initial_pos {
        if path_result[pos as usize] == 'X' {

            result += check_loop(initial_pos, pos, width, &mut contents, &masks)
        }
    }
    for pos in initial_pos+1..path_result.len() as isize {
        if path_result[pos as usize] == 'X' {

            result += check_loop(initial_pos, pos, width, &mut contents, &masks)
        }
    }

    println!("result = {result}");
    
    Ok(())
}

fn check_loop(initial_pos: isize, block_pos: isize, w: isize, content: &mut [char], masks: &[Mask]) -> i32{
    let mut counts = 0;
    let mut dir = Dirs::Up;

    let mut pos = initial_pos;

    let mut mask = get_mask(masks, &dir);

    let mut visited = HashSet::new();

    let save_state = content[block_pos as usize];
    content[block_pos as usize] = 'O';

    loop {
        let looped = !visited.insert((pos, dir.clone()));
        if looped {
            counts = 1;
            break;
        }
        let look_ahead = mask.apply(pos, w, content);
        if let Some(char) = look_ahead {
            match char.first().unwrap() {
                '#' | 'O' => { 
                    dir.rotate();
                    mask = get_mask(masks, &dir);
                },
                _ => { pos = advance(pos, &dir, w); },
            }
        } else {
            break;
        }
    }

    content[block_pos as usize] = save_state;

    counts
}

pub fn answer() -> Result<(), io::Error>{
    println!("Part1:");
    part1()?;

    println!("Part2:");
    part2()?;

    Ok(())
}
