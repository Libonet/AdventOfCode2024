use std::collections::{HashSet, VecDeque};
use std::{fs::read_to_string, io};
use crate::matrix::{Matrix, Mask};
use crate::position::Pos;

type Price = u64;

#[derive(Debug)]
struct Region {
    plant: char,
    area: u64,
    perimeter: u64,
}

impl Region {
    fn new(plant: char) -> Self {
        Self {
            plant,
            area: 0,
            perimeter: 0,
        }
    }

    fn get_price(&self) -> Price {
        self.area * self.perimeter
    }
}

type Input = Matrix<char>;

fn parse(contents: &str) -> Input {
    let width = contents.find("\n").unwrap();
    let rows = contents.replace("\n", "").chars().collect();

    Matrix::new(rows, width)
}

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day12.txt")?; // input/dayxx.txt

    let input = parse(&contents);
    
    println!("Part1:");
    let part1_res = part1(&input);
    println!("result = {part1_res}");

    println!("Part2:");
    let part2_res = part2(&input);
    println!("result = {part2_res}");

    Ok(())
}

fn part1(matrix: &Input) -> Price {
    let mut visited = Matrix::with_capacity(matrix.row_count(), matrix.width());
    for _val in matrix {
        visited.push(false);
    }

    let mut regions: Vec<Region> = Vec::new();
    for (val, pos) in matrix.give_pos() {
        if !*visited.get_pos(&pos).unwrap() {
            region_expansion(matrix, &mut visited, &mut regions, val, &pos);
        }
    }

    regions.iter().map(|region| region.get_price()).sum()
}

fn make_mask() -> Mask {
    // all cardinal directions.
    // .X.
    // XPX
    // .X.
    Mask::new(vec![Pos(-1,0),Pos(1,0),Pos(0,-1),Pos(0,1),])
}

fn region_expansion(
    matrix: &Matrix<char>,
    visited: &mut Matrix<bool>,
    regions: &mut Vec<Region>,
    val: &char, pos: &Pos
) {
    let cardinal_mask = make_mask();
    let mut region = Region::new(*val);

    let mut members: VecDeque<Pos> = VecDeque::new();
    members.push_back(*pos);
    while !members.is_empty() {
        let curr_pos = members.pop_front().unwrap();
        if !*visited.get_pos(&curr_pos).unwrap() {
            *visited.get_mut_pos(&curr_pos).unwrap() = true;
            region.area += 1;
            
            let neighbours = cardinal_mask.apply(curr_pos, matrix);

            for (i, val) in neighbours.iter().enumerate() {
                if let Some(chr) = val {
                    if **chr == region.plant {
                        let offset = match i {
                            0 => Pos(-1,0),
                            1 => Pos(1,0),
                            2 => Pos(0,-1),
                            3 => Pos(0,1),
                            _ => unreachable!(),
                        };

                        members.push_back(curr_pos + offset);
                    } else {
                        region.perimeter += 1;
                    }
                } else {
                    region.perimeter += 1;
                }
            }
        }
    }

    regions.push(region);
}

fn part2(matrix: &Input) -> Price {
    let mut visited = Matrix::with_capacity(matrix.row_count(), matrix.width());
    for _val in matrix {
        visited.push(false);
    }

    let mut regions: Vec<Region> = Vec::new();
    for (val, pos) in matrix.give_pos() {
        if !*visited.get_pos(&pos).unwrap() {
            region_expansion_discounted(matrix, &mut visited, &mut regions, val, &pos);
        }
    }

    regions.iter().map(|region| region.get_price()).sum()
}

fn region_expansion_discounted(
    matrix: &Matrix<char>,
    visited: &mut Matrix<bool>,
    regions: &mut Vec<Region>,
    val: &char, pos: &Pos
) {
    let cardinal_mask = make_mask();
    let mut region = Region::new(*val);
    let mut perimeter: HashSet<(Pos, Dir)> = HashSet::new();

    let mut members: VecDeque<Pos> = VecDeque::new();
    members.push_back(*pos);
    while !members.is_empty() {
        let curr_pos = members.pop_front().unwrap();
        if !*visited.get_pos(&curr_pos).unwrap() {
            *visited.get_mut_pos(&curr_pos).unwrap() = true;
            region.area += 1;
            
            let neighbours = cardinal_mask.apply(curr_pos, matrix);

            for (i, val) in neighbours.iter().enumerate() {
                let (offset, off_dir) = match i {
                    0 => (Pos(-1,0), Dir::Up),
                    1 => (Pos(1,0), Dir::Down),
                    2 => (Pos(0,-1), Dir::Left),
                    3 => (Pos(0,1), Dir::Right),
                    _ => unreachable!(),
                };
                let relative_pos = curr_pos + offset;

                if let Some(chr) = val {
                    if **chr == region.plant {
                        members.push_back(relative_pos);
                    } else {
                        region.perimeter += 1;
                        perimeter.insert((relative_pos, off_dir));
                    }
                } else {
                    region.perimeter += 1;
                    perimeter.insert((relative_pos, off_dir));
                }
            }
        }
    }
    
    let perimeter = perimeter.into_iter().collect::<Vec<(Pos, Dir)>>();

    for i in 0..perimeter.len() {
        let (p1, d1) = &perimeter[i];

        for (p2, d2) in perimeter.iter().skip(i) {
            let Pos(a,b) = *p1 - *p2;
            if d1 == d2 && a.abs() + b.abs() == 1 {
                region.perimeter -= 1;
            }
        }
    }

    regions.push(region);
}

#[derive(PartialEq, Eq, Debug, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        let input = parse(input);
        let result = part1(&input);

        assert_eq!(result, 1930);
    }

    #[test]
    fn test_part2() {
        let input = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        let input = parse(input);
        let result = part2(&input);

        assert_eq!(result, 1206);
    }
}
