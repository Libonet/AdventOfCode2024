use std::collections::BinaryHeap;
use std::time::Instant;
use std::{fs::read_to_string, io};

use crate::matrix::Matrix;
use crate::position::{Dir, UPos};
use crate::scanner::Scanner;

type Input = Vec<UPos>;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day18.txt")?; // input/dayxx.txt
    
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
    let mut scanner = Scanner::new(&contents);

    let mut list = Vec::new();
    while !scanner.is_done() {
        let y = scanner.try_u32().unwrap() as usize;

        // pop the ','
        scanner.pop();
        
        let x = scanner.try_u32().unwrap() as usize;

        list.push(UPos(x,y));

        // pop the '\n'
        scanner.pop();
    }

    list
}

type Steps = u32;

fn part1(input: &Input) -> Steps {
    let start = UPos(0,0);
    let end = UPos(70,70);

    let mut map = Matrix::with_capacity(71, 71, '.');

    simulate_drops(&mut map, input, 1024);

    let (cost_map, _tiles) = homemade_dijkstra(&mut map, start, end);

    cost_map[end].cost
}

fn simulate_drops(map: &mut Matrix<char>, drops: &Input, size: usize) {
    assert!(size <= drops.len());
    for upos in drops.iter().take(size) {
        map[*upos] = '#';
    }
}

type Cost = u32;

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

fn homemade_dijkstra(map: &mut Matrix<char>, start: UPos, end: UPos) -> (Matrix<CellInfo>, Vec<Vec<UPos>>) {
    let mut map_costs: Matrix<CellInfo> = Matrix::with_capacity(map.row_count(), map.width(), CellInfo::default());
    map_costs[start] = CellInfo::new(0);

    let mut heap: BinaryHeap<Status> = BinaryHeap::new();

    let all_dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
    
    let curr_cell = Status { cost:0, pos:start, dir:Dir::Right, path: vec![start] };
    heap.push(curr_cell);

    let mut paths: Vec<Vec<UPos>> = Vec::new();
    while !heap.is_empty() {
        let curr_status = heap.pop().unwrap();

        if curr_status.pos == end {
            map_costs[end].cost = curr_status.cost;

            paths.push(curr_status.path);
            continue;
        }

        for dir in &all_dirs {
            if dir.opposite(&curr_status.dir) {
                continue;
            }

            let res = map_costs.look_ahead(&curr_status.pos, dir);
            if res.is_some() {
                let (next_cell, next_pos) = res.unwrap();
                if map[next_pos] == '#' {
                    // Cant go through walls :c
                    continue;
                }
                let dir_cost = curr_status.cost + 1;
                if next_cell.cost > dir_cost {
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
                }
            }
        }
    }

    (map_costs, paths)
}

fn part2(input: &Input) -> (usize, usize) {
    let start = UPos(0,0);
    let end = UPos(70,70);

    let map = Matrix::with_capacity(71, 71, '.');

    // binary search the last moment we could reach the end

    let mut l = 1025;
    let mut r = input.len();

    let mut m = (r + l) / 2;

    while l != m {
        let mut clone = map.clone();
        simulate_drops(&mut clone, input, m);

        let (_cost_map, paths) = homemade_dijkstra(&mut clone, start, end);

        if paths.is_empty() {
            r = m-1;
        } else {
            l = m+1;
        }

        m = (r + l) / 2;        
    }

    let last = input[m+1];
    (last.1, last.0)
}
