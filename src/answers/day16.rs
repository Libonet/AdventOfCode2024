use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}, fs::read_to_string, io};

use crate::{matrix::Matrix, position::{Dir, Pos, UPos}};

type Input = (Matrix<char>, UPos, UPos);
type Cost = u64;

#[derive(Debug, Eq, PartialEq)]
struct Status {
    cost: Cost,
    pos: UPos,
    dir: Dir,
    path: Vec<UPos>,
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct CellInfo {
    cost: Cost,
}

impl CellInfo {
    fn new(cost: Cost) -> Self {
        Self { cost }
    }

    fn default() -> Self {
        Self { cost: Cost::MAX }
    }
}

fn part1(input: &Input) -> Cost {
    let clone_input = input.clone();
    let (start, end) = (clone_input.1, clone_input.2);

    if start == end {
        return 0;
    }

    let (map_costs, _tiles) = homemade_dijkstra(clone_input);

    map_costs[end].cost
}

fn homemade_dijkstra(input: Input) -> (Matrix<CellInfo>, HashSet<UPos>) {
    let (mut map, start, end) = input;

    let mut map_costs: Matrix<CellInfo> = Matrix::with_capacity(map.row_count(), map.width(), CellInfo::default());
    map_costs[start] = CellInfo::new(0);

    let mut heap: BinaryHeap<Status> = BinaryHeap::new();

    let all_dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
    
    let curr_cell = Status { cost:0, pos:start, dir:Dir::Right, path: vec![start] };
    heap.push(curr_cell);

    let mut tiles: HashSet<UPos> = HashSet::new();
    while !heap.is_empty() {
        let curr_status = heap.pop().unwrap();

        if curr_status.pos == end {
            match curr_status.cost.cmp(&map_costs[end].cost) {
                Ordering::Less | Ordering::Equal => {
                    map_costs[end].cost = curr_status.cost;

                    tiles.extend(curr_status.path.into_iter());
                    continue;
                },
                Ordering::Greater => continue,
            }
        }

        for dir in &all_dirs {
            let res = map_costs.look_ahead(&curr_status.pos, dir);
            if res.is_some() {
                let (next_cell, next_pos) = res.unwrap();
                if map[next_pos] == '#' {
                    // Cant go through walls :c
                    continue;
                }
                let rot_cost = rotation_cost(curr_status.dir, *dir);
                if rot_cost == 2000 {
                    // We dont want to go backwards
                    continue;
                }

                let dir_cost = curr_status.cost + rot_cost + 1;
                match next_cell.cost.cmp(&dir_cost) {
                    Ordering::Greater | Ordering::Equal => {
                        let (next_cell, next_pos) = map_costs.look_ahead_mut(&curr_status.pos, dir).unwrap();
                        next_cell.cost = dir_cost;

                        let mut new_path = (curr_status.path).clone();
                        new_path.push(next_pos);
                        let next_status = Status { 
                            cost: dir_cost,
                            pos: next_pos,
                            dir: *dir,
                            path: new_path,    
                        };
                        heap.push(next_status);
                    },
                    Ordering::Less => {
                        let turning_point = next_cell.cost + 1000;

                        match turning_point.cmp(&dir_cost) {
                            Ordering::Greater | Ordering::Equal => {
                                let mut new_path = (curr_status.path).clone();
                                new_path.push(next_pos);
                                let next_status = Status {
                                    cost: dir_cost,
                                    pos: next_pos,
                                    dir: *dir,
                                    path: new_path,
                                };
                                heap.push(next_status);
                            },
                            Ordering::Less => (),
                        }
                    }
                }
            }
        }
    }

    map[end] = 'O';

    for tile in &tiles {
        map[*tile] = 'O';
    }
    
    (map_costs, tiles)
}

fn rotation_cost(initial: Dir, next: Dir) -> Cost {
    let initial_pos: Pos = initial.into();
    let next_pos: Pos = next.into();
    match initial_pos + next_pos {
        Pos(0,0) => 2000,
        Pos(0,_) | Pos(_,0) => 0,
        _ => 1000,
    }
}

type Count = usize;

fn part2(input: &Input) -> Count {
    let clone_input = input.clone();
    let (start, end) = (clone_input.1, clone_input.2);

    if start == end {
        return 0;
    }

    let (_map_costs, tiles) = homemade_dijkstra(clone_input);

    // Now that we found all the best paths, lets count the tiles

    tiles.len()
}

pub fn answer() -> Result<(), io::Error>{
    let input = read_to_string("input/day16.txt")?; // input/dayxx.txt
    
    let input = parse(input);

    println!("Part1:");
    let part1_res = part1(&input);
    println!("result = {part1_res}");

    println!("Part2:");
    let part2_res = part2(&input);
    println!("result = {part2_res}");

    Ok(())
}

fn parse(input: String) -> Input {
    let width = input.find("\n").unwrap();
    let rows = input.replace("\n", "");

    let start = rows.find("S").unwrap();

    let x = start / width;
    let y = start % width;
    let spos = UPos(x,y);

    let end = rows.find("E").unwrap();

    let x = end / width;
    let y = end % width;
    let epos = UPos(x,y);

    let rows = rows.chars().collect();
    let map = Matrix::new(rows, width);

    (map, spos, epos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################".to_string();

        let input = parse(input);

        let result = part2(&input);

        assert_eq!(result, 64);
    }

    #[test]
    fn test_part2_smaller() {
        let input = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############".to_string();

        let input = parse(input);

        let result = part2(&input);

        assert_eq!(result, 45);
    }
}
