use std::{fs::read_to_string, io};

use crate::{matrix::Matrix, position::{Pos, UPos}};

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn get_offset(&self) -> Pos {
        match self {
            Dir::Up => Pos(-1,0),
            Dir::Down => Pos(1,0),
            Dir::Left => Pos(0,-1),
            Dir::Right => Pos(0,1),
        }
    }
}

type Input = (UPos, Matrix<char>, Vec<Dir>);
type Cost = i64;

fn part1(contents: &Input) -> Cost {
    let mut robot_pos = contents.0;
    let mut map = contents.1.clone();
    let moves = &contents.2;

    for dir in moves {
        let possible_pos = try_move(&robot_pos, &mut map, dir);
        if let Some(new_pos) = possible_pos {
            robot_pos = new_pos;
        }
    }

    calculate_costs(&map)
}

fn try_move(pos: &UPos, map: &mut Matrix<char>, dir: &Dir) -> Option<UPos> {
    let offset = dir.get_offset();

    let next_pos: Pos = Pos::from(*pos) + offset;
    let next_pos: Result<UPos, _> = next_pos.try_into();
    let next_pos = match next_pos {
        Ok(pos) => pos,
        Err(_e) => return None,
    };

    match map.get(&next_pos) {
        Some(char) => {
            match char {
                '#' => None,
                'O' => {
                    let opt = try_move(&next_pos, map, dir);
                    match opt {
                        Some(_new_pos) => {
                            let actual_val = map[*pos];
                            map[next_pos] = actual_val;
                            map[*pos] = '.';
                            Some(next_pos)
                        },
                        None => None,
                    }
                },
                _ => {
                    let actual_val = map[*pos];
                    map[next_pos] = actual_val;
                    map[*pos] = '.';
                    Some(next_pos)
                }
            }
        },
        None => None,
    }
}

fn calculate_costs(map: &Matrix<char>) -> Cost {
    map
        .give_upos()
        .map(|(val, upos)| {
            if *val == 'O' {
                (100 * upos.0 + upos.1) as Cost
            } else {
                0
            }
        })
        .sum()
}

fn part2(contents: &Input) -> Cost {
    let mut robot_pos = contents.0;
    let mut map = contents.1.clone();
    let moves = &contents.2;

    for dir in moves {
        let next_pos = look_ahead(&robot_pos, dir);
        if let Some(pos) = next_pos {
            if able_move_wide(&pos, &map, dir) {
                move_wide(&robot_pos, &mut map, dir);
                robot_pos = pos;
            }
        }
    }

    calculate_costs_wide(&map)
}

fn look_ahead(pos: &UPos, dir: &Dir) -> Option<UPos> {
    let offset = dir.get_offset();

    let next_pos: Pos = Pos::from(*pos) + offset;
    next_pos.try_into().ok()
}

fn able_move_wide(pos: &UPos, map: &Matrix<char>, dir: &Dir) -> bool {
    match dir {
        Dir::Left | Dir::Right => can_move_hori(pos, map, dir),
        Dir::Up | Dir::Down => can_move_vert(pos, map, dir),
    }
}

fn can_move_hori(next_pos: &UPos, map: &Matrix<char>, dir: &Dir) -> bool {
    match map.get(next_pos) {
        Some(val) => {
            match val {
                '#' => false,
                '[' | ']' => {
                    let next = look_ahead(next_pos, dir);
                    if let Some(pos) = next {
                        can_move_hori(&pos, map, dir)
                    } else {
                        false
                    }
                },
                _ => {
                    true
                },
            }
        },
        None => false
    }
}

fn can_move_vert(next_pos: &UPos, map: &Matrix<char>, dir: &Dir) -> bool {
    match map.get(next_pos) {
        Some(val) => {
            match val {
                '#' => false,
                '[' => {
                    let next = look_ahead(next_pos, dir);
                    if let Some(pos) = next {
                        let move_l = can_move_vert(&pos, map, dir);
                        let right = look_ahead(&pos, &Dir::Right);
                        if let Some(pos) = right {
                            move_l && can_move_vert(&pos, map, dir)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                },
                ']' => {
                    let next = look_ahead(next_pos, dir);
                    if let Some(pos) = next {
                        let move_r = can_move_vert(&pos, map, dir);
                        let left = look_ahead(&pos, &Dir::Left);
                        if let Some(pos) = left {
                            move_r && can_move_vert(&pos, map, dir)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                _ => {
                    true
                },
            }
        },
        None => false
    }
}

fn move_wide(pos: &UPos, map: &mut Matrix<char>, dir: &Dir) {
    match dir {
        Dir::Left | Dir::Right => move_block_hori(pos, map, dir),
        Dir::Up | Dir::Down => {
            let next_pos = look_ahead(pos, dir);
            let next_pos = match next_pos {
                Some(pos) => pos,
                None => return,
            };
            
            match map[next_pos] {
                '[' => {
                    move_block_vert(&next_pos, map, dir);
                    map[next_pos] = '@';
                    map[*pos] = '.';
                },
                ']' => {
                    let next_left = look_ahead(&next_pos, &Dir::Left).unwrap();
                    move_block_vert(&next_left, map, dir);
                    map[next_pos] = '@';
                    map[*pos] = '.';
                },
                '.' => {
                    map[next_pos] = '@';
                    map[*pos] = '.';
                }
                _ => (),
            }
        },
    }
}

fn move_block_hori(pos: &UPos, map: &mut Matrix<char>, dir: &Dir) {
    let next_pos = look_ahead(pos, dir);
    let next_pos = match next_pos {
        Some(pos) => pos,
        None => return,
    };

    if let Some(val) = map.get(&next_pos) {
        match val {
            '#' => (),
            '[' | ']' => {
                move_block_hori(&next_pos, map, dir);
                let curr_val = map[*pos];
                map[next_pos] = curr_val;
                map[*pos] = '.';
            },
            _ => {
                let curr_val = map[*pos];
                map[next_pos] = curr_val;
                map[*pos] = '.';
            },
        }
    }
}

fn move_block_vert(pos: &UPos, map: &mut Matrix<char>, dir: &Dir) {
    let next_left = look_ahead(pos, dir);
    let next_left = match next_left {
        Some(pos) => pos,
        None => return,
    };
    
    let right_offset = look_ahead(pos, &Dir::Right).unwrap();
    let next_right = look_ahead(&right_offset, dir);
    let next_right = match next_right {
        Some(pos) => pos,
        None => return,
    };

    if let (Some(left), Some(right)) = (map.get(&next_left), map.get(&next_right)) {
        match (left, right) {
            ('#', _) => (),
            (_, '#') => (),
            ('[', ']') => {
                move_block_vert(&next_left, map, dir);

                let curr_val = map[*pos];
                map[next_left] = curr_val;
                map[*pos] = '.';
                let curr_val = map[right_offset];
                map[next_right] = curr_val;
                map[right_offset] = '.';
            },
            (']', '[') => {
                move_block_vert(&next_right, map, dir);
                let next2_left = look_ahead(&next_left, &Dir::Left).unwrap();
                move_block_vert(&next2_left, map, dir);

                let curr_val = map[*pos];
                map[next_left] = curr_val;
                map[*pos] = '.';
                let curr_val = map[right_offset];
                map[next_right] = curr_val;
                map[right_offset] = '.';
            },
            (']', _) => {
                let next2_left = look_ahead(&next_left, &Dir::Left).unwrap();
                move_block_vert(&next2_left, map, dir);

                let curr_val = map[*pos];
                map[next_left] = curr_val;
                map[*pos] = '.';
                let curr_val = map[right_offset];
                map[next_right] = curr_val;
                map[right_offset] = '.';
            },
            (_, '[') => {
                move_block_vert(&next_right, map, dir);

                let curr_val = map[*pos];
                map[next_left] = curr_val;
                map[*pos] = '.';
                let curr_val = map[right_offset];
                map[next_right] = curr_val;
                map[right_offset] = '.';
            },
            _ => {
                let curr_val = map[*pos];
                map[next_left] = curr_val;
                map[*pos] = '.';
                let curr_val = map[right_offset];
                map[next_right] = curr_val;
                map[right_offset] = '.';
            },
        }
    }
}

fn calculate_costs_wide(map: &Matrix<char>) -> Cost {
    map
        .give_upos()
        .map(|(val, upos)| {
            if *val == '[' {
                (100 * upos.0 + upos.1) as Cost
            } else {
                0
            }
        })
        .sum()
}

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day15.txt")?; // input/dayxx.txt
    
    let p1_input = parse(&contents);

    println!("Part1:");
    let part1_res = part1(&p1_input);
    println!("result = {part1_res}");

    let p2_input = parse_p2(&contents);

    println!("Part2:");
    let part2_res = part2(&p2_input);
    println!("result = {part2_res}");

    Ok(())
}

fn parse(contents: &str) -> Input {
    let width = contents.find("\n").unwrap();
    let (map, rest) = contents.split_once("\n\n").unwrap();

    let map = map.replace("\n", "");

    let robot_pos = map.find("@").unwrap();

    let x = robot_pos / width;
    let y = robot_pos % width;

    let robot_pos = UPos(x,y);

    let rows:Vec<char> = map.chars().collect();

    let map = Matrix::new(rows, width);
    let rest = rest.replace("\n", "");

    let rest = rest
        .chars()
        .map(|c| {
            match c {
                '^' => Dir::Up,
                'v' => Dir::Down,
                '<' => Dir::Left,
                '>' => Dir::Right,
                _ => unreachable!("input is clean"),
            }
        })
        .collect();

    (robot_pos, map, rest)
}

fn parse_p2(contents: &str) -> Input {
    let width = contents.find("\n").unwrap() * 2;
    let (map, rest) = contents.split_once("\n\n").unwrap();

    let map = map.replace("\n", "");

    let robot_pos = map.find("@").unwrap() * 2;

    let x = robot_pos / width;
    let y = robot_pos % width;

    let robot_pos = UPos(x,y);

    let rows:Vec<char> = map.chars().flat_map(|c| {
        match c {
            'O' => ['[',']'],
            '@' => ['@','.'],
            rest => [rest,rest],
        }
    }).collect();

    let map = Matrix::new(rows, width);
    let rest = rest.replace("\n", "");

    let rest = rest
        .chars()
        .map(|c| {
            match c {
                '^' => Dir::Up,
                'v' => Dir::Down,
                '<' => Dir::Left,
                '>' => Dir::Right,
                _ => unreachable!("input is clean"),
            }
        })
        .collect();

    (robot_pos, map, rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let input = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        let contents = parse(input);

        let result = part1(&contents);

        assert_eq!(result, 2028);
    }

    #[test]
    fn big_example() {
        let contents = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let contents = parse(contents);

        let result = part1(&contents);

        assert_eq!(result, 10092);
    }

    #[test]
    fn big_example_part2() {
        let contents = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let contents = parse_p2(contents);

        let result = part2(&contents);

        assert_eq!(result, 9021);
    }
}
