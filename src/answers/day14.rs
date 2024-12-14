use std::{fs::read_to_string, io};
use crate::{matrix::{Matrix, Mask}, position::{Pos, UPos}, scanner::Scanner};

type Input = Vec<Robot>;
type SafetyFactor = i32;

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: UPos,
    v: Pos,
}

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day14.txt")?; // input/dayxx.txt
    
    let robots = parse(&contents);

    println!("Part1:");
    let part1_res = part1(&robots);
    println!("result = {part1_res}");

    println!("Part2:");
    let part2_res = part2(&robots);
    println!("result = {part2_res}");

    Ok(())
}

fn part1(input: &Input) -> SafetyFactor {
    let matrix: Matrix<i32> = Matrix::with_capacity(103, 101, 0);

    let mut quadrants = [0,0,0,0,0];
    for robot in input {
        let end_upos = simulate_robot(&matrix, robot, 100);

        let quadrant = get_quadrant(&end_upos, matrix.row_count(), matrix.width());
        quadrants[quadrant] += 1;
    }

    quadrants.into_iter().take(4).reduce(|acc, v| acc * v).unwrap() // safety factor
}

fn simulate_robot(matrix: &Matrix<i32>, robot: &Robot, seconds: i32) -> UPos {
    let mut upos = robot.p;
    let vel = robot.v * seconds;
    
    upos = safe_add(&upos, &vel, matrix.row_count(), matrix.width());

    upos
}

fn safe_add(upos: &UPos, pos: &Pos, height: usize, width: usize) -> UPos {
    let x = (upos.0 as i32 + pos.0).rem_euclid(height as i32);
    let y = (upos.1 as i32 + pos.1).rem_euclid(width as i32);

    UPos(x as usize,y as usize)
}

fn get_quadrant(upos: &UPos, height: usize, width: usize) -> usize {
    let mid_h = height / 2;
    let mid_w = width / 2;
    let mid_h1 = mid_h + 1;
    let mid_w1 = mid_w + 1;

    if (0..mid_h).contains(&upos.0) && (0..mid_w).contains(&upos.1) {
        0
    } else if (0..mid_h).contains(&upos.0) && (mid_w1..width).contains(&upos.1) {
        1
    } else if (mid_h1..height).contains(&upos.0) && (0..mid_w).contains(&upos.1) {
        2
    } else if (mid_h1..height).contains(&upos.0) && (mid_w1..width).contains(&upos.1) {
        3
    } else {
        4
    }
}
    
fn part2(input: &Input) -> i32 {
    let mut robots = input.clone();

    // xxxx
    // xxxx
    // xxxx
    // xxxx
    let mask = Mask::new(vec![
        Pos(0,0),Pos(0,1),Pos(0,2),Pos(0,3),
        Pos(1,0),Pos(1,1),Pos(1,2),Pos(1,3),
        Pos(2,0),Pos(2,1),Pos(2,2),Pos(2,3),
        Pos(3,0),Pos(3,1),Pos(3,2),Pos(3,3),
    ]);

    let mut matrix: Matrix<i32> = Matrix::with_capacity(103, 101, 0);
    for robot in &robots {
        matrix[robot.p] += 1;
    }

    let mut best_times = Vec::new();
    for sec in 1..10403 {
        for robot in &mut robots {
            matrix[robot.p] -= 1;
            let end_pos = simulate_robot(&matrix, robot, 1);
            matrix[end_pos] += 1;
            robot.p = end_pos;
        }
        for (_val, pos) in matrix.give_pos() {
            let mask_res = mask.apply(pos, &matrix);
            let check: i32 = mask_res
                .iter()
                .map(|opt| {
                    match *opt {
                        Some(val) if *val == 1 => 1,
                        _ => 0,
                    }
                })
                .sum();

            if check == 16 {
                best_times.push(sec);
                break;
            }
        }
        if best_times.len() >= 5 {
            break;
        }
    }

    for i in &best_times {
        let mut matrix = Matrix::with_capacity(103,101,0);
        for robot in input {
            let end_pos = simulate_robot(&matrix, robot, *i);
            matrix[end_pos] += 1;
        }
        println!("after {}s", i);
        let mut debug_matrix = Matrix::with_capacity(103,101,' ');
        for (val, upos) in matrix.give_upos() {
            match *val {
                1 => debug_matrix[upos] = '*',
                2.. => debug_matrix[upos] = '#',
                _ => (),
            }
        }
        println!("{debug_matrix}");
    }

    *best_times.first().unwrap()
}

/*
fn standard_deviation(num: &[i64]) -> i64 {
    let mean = num.iter().sum::<i64>() / num.len() as i64;
    num.iter().map(|x| (x - mean).pow(2)).sum::<i64>() / num.len() as i64
}
*/

fn parse(contents: &str) -> Input {
    let mut robots = Vec::new();
    for line in contents.lines() {
        let mut scanner = Scanner::new(line);

        if !scanner.try_pattern("p=") {
            continue;
        }

        let py = scanner.try_u32();
        let py = match py {
            Some(num) => num as usize,
            None => continue,
        };

        if !scanner.take(&',') {
            continue;
        }

        let px = scanner.try_u32();
        let px = match px {
            Some(num) => num as usize,
            None => continue,
        };

        
        if !scanner.try_pattern(" v=") {
            continue;
        }

        let vy = scanner.try_i32();
        let vy = match vy {
            Some(num) => num,
            None => continue,
        };

        if !scanner.take(&',') {
            continue;
        }

        let vx = scanner.try_i32();
        let vx = match vx {
            Some(num) => num,
            None => continue,
        };

        let robot = Robot { p: UPos(px,py), v: Pos(vx,vy) };

        robots.push(robot);
    }

    robots
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation() {
        let matrix = Matrix::with_capacity(7, 11, 0);

        let robot = Robot { p: UPos(4,2), v: Pos(-3,2) };

        let end_pos = simulate_robot(&matrix, &robot, 5);

        assert_eq!(end_pos, UPos(3,1));
    }
}
