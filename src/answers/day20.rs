use std::fmt::Display;
use std::{fs::read_to_string, io};
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

use crate::matrix::{self, Matrix};
use crate::position::{Dir, UPos};

type Input = (Matrix<char>, UPos, UPos);

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day20.txt")?; // input/dayxx.txt
    
    let input = parse(contents);
    
    println!("Part1:");
    let now = Instant::now();
    let part1_res = part1(&input);
    let elapsed = now.elapsed();
    println!("result = {part1_res}");
    println!("Time taken: {:.2?}", elapsed);

    println!("Part2:");
    let now = Instant::now();
    let part2_res = part2(&input);
    let elapsed = now.elapsed();
    println!("result = {part2_res:?}");
    println!("Time taken: {:.2?}", elapsed);

    Ok(())
}

fn parse(contents: String) -> Input {
    let width = contents.find("\n").unwrap();
    let rows = contents.replace("\n", "");

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

type Count = usize;

fn part1(input: &Input) -> Count {
    let (mut map, start, end) = input.clone();

    // get the time from start to end with no cheats
    let map_costs = homemade_dijkstra(&mut map, start, end);

    find_cheats(&map, &map_costs, start, end, 2, 100)
}

type Cost = u32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CellInfo {
    cost: Cost,
    dir: Dir,
}

impl Display for CellInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cost = self.cost;
        let dir = self.dir;
        write!(f, "{{{cost},{dir}}}")
    }
}

impl CellInfo {
    fn new(cost: Cost, dir: Dir) -> Self {
        Self { cost, dir }
    }

    fn default() -> Self {
        Self { cost: Cost::MAX, dir: Dir::Right }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Status {
    cost: Cost,
    pos: UPos,
    dir: Dir,
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

fn homemade_dijkstra(map: &mut Matrix<char>, start: UPos, end: UPos) -> Matrix<CellInfo> {
    let mut map_costs: Matrix<CellInfo> = Matrix::with_capacity(map.row_count(), map.width(), CellInfo::default());
    map_costs[start] = CellInfo::new(0, Dir::Right);

    let mut heap: BinaryHeap<Status> = BinaryHeap::new();

    let all_dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
    
    let curr_cell = Status { cost:0, pos:start, dir:Dir::Right };
    heap.push(curr_cell);

    while !heap.is_empty() {
        let curr_status = heap.pop().unwrap();

        if curr_status.pos == end {
            map_costs[end].cost = curr_status.cost;
            continue;
        }

        for dir in &all_dirs {
            if dir.opposite(&curr_status.dir) {
                continue;
            }

            let res = map_costs.look_ahead_mut(&curr_status.pos, dir);
            let mut next_dir = Dir::Right;
            if res.is_some() {
                let (next_cell, next_pos) = res.unwrap();
                if map[next_pos] == '#' {
                    // Cant go through walls :c
                    continue;
                }

                next_dir = *dir;
                let dir_cost = curr_status.cost + 1;
                if next_cell.cost > dir_cost {
                    let (next_cell, next_pos) = map_costs.look_ahead_mut(&curr_status.pos, dir).unwrap();
                    next_cell.cost = dir_cost;

                    let next_status = Status { 
                        cost: dir_cost,
                        pos: next_pos,
                        dir: *dir,
                    };
                    heap.push(next_status);
                }
            }
            map_costs[curr_status.pos].dir = next_dir;
        }
    }

    map_costs
}

fn find_cheats(
    map: &Matrix<char>,
    map_costs: &Matrix<CellInfo>, 
    start: UPos, 
    end: UPos, 
    max_cheat_distance: usize,
    savings: Cost,
) -> Count {
    let mut good_cheats = 0;
    // Move through the map_costs following the dir
    let mut status = Status { cost:0, pos:start, dir:Dir::Right };
    while status.pos != end {
        let cell = &map_costs[status.pos];

        status.cost = cell.cost;
        status.dir = cell.dir;

        let mut endings = HashSet::new();

        // Move to wherever we can reach
        let valid_ends = map.give_upos()
            .filter_map(|(val, upos)| {
                if *val == '#' || status.pos == upos {
                    return None;
                }
                let distance = matrix::manhattan_distance(&status.pos, &upos);
                if distance <= max_cheat_distance {
                    Some(upos)
                } else {
                    None
                }
            });
        
        endings.extend(valid_ends);

        //let mut dedup_endings = Vec::new();
        //let find_upos = |list: &Vec<(UPos,u32)>, upos: &UPos| {
        //    for (i, (v,_d)) in list.iter().enumerate() {
        //        if *v == *upos {
        //            return Some(i);
        //        }
        //    }
        //
        //    None
        //};
        //for (upos, cheat_cost) in endings {
        //    match find_upos(&dedup_endings, &upos) {
        //        Some(i) => {
        //            let (_old_upos, old_cost) = dedup_endings[i];
        //            if cheat_cost < old_cost {
        //                dedup_endings[i].1 = cheat_cost;
        //            }
        //        },
        //        None => dedup_endings.push((upos, cheat_cost)),
        //    }
        //}
        for upos in endings {
            let cheat_cost = matrix::manhattan_distance(&status.pos, &upos) as Cost;
            if map_costs[upos].cost > status.cost
            && map_costs[upos].cost - status.cost > savings + (cheat_cost - 1) {
                good_cheats += 1;
            }
        }

        let (_val, upos) = map.look_ahead(&status.pos, &status.dir).unwrap();
        status.pos = upos;
    }

    good_cheats
}

fn part2(input: &Input) -> Count {
    let (mut map, start, end) = input.clone();

    // get the time from start to end with no cheats
    let map_costs = homemade_dijkstra(&mut map, start, end);

    find_cheats(&map, &map_costs, start, end, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        let input = parse(contents.to_string());

        let (mut map, start, end) = input.clone();

        // get the time from start to end with no cheats
        let map_costs = homemade_dijkstra(&mut map, start, end);

        let result = find_cheats(&map, &map_costs, start, end, 2, 20);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let contents = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        let input = parse(contents.to_string());

        let (mut map, start, end) = input.clone();

        // get the time from start to end with no cheats
        let map_costs = homemade_dijkstra(&mut map, start, end);

        let result = find_cheats(&map, &map_costs, start, end, 20, 50);

        assert_eq!(result, 285);
    }
}
