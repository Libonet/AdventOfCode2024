use std::{fs::read_to_string, io};
use crate::{matrix::Matrix, scanner::Scanner, position::Pos};

type Input = Vec<Matrix<i128>>;
type Tokens = u128;

// To solve this problem we need to find a solution to
// A*x = B
// then
// x = A^-1 * B
fn part1(machines: &Input) -> Tokens {
    machines.iter()
        .filter_map(|augmented| {
            solve_2by2(augmented, 0)
        })
        .sum()
}

fn part2(machines: &Input) -> Tokens {
    machines.iter()
        .filter_map(|augmented| {
            solve_2by2(augmented, 10_000_000_000_000)
        })
        .sum()
}

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day13.txt")?;
    
    let matrix = parse(&contents);
    
    println!("Part1:");
    let part1_res = part1(&matrix);
    println!("result = {part1_res}");

    println!("Part2:");
    let part2_res = part2(&matrix);
    println!("result = {part2_res}");

    Ok(())
}

// matrix:
// ax bx px
// ay by py
fn solve_2by2<T: Into<i128> + Copy>(matrix: &Matrix<T>, offset: i128) -> Option<Tokens> {
    let ax: i128 = matrix[Pos(0,0)].into();
    let bx: i128 = matrix[Pos(0,1)].into();
    let px: i128 = matrix[Pos(0,2)].into();
    let ay: i128 = matrix[Pos(1,0)].into();
    let by: i128 = matrix[Pos(1,1)].into();
    let py: i128 = matrix[Pos(1,2)].into();

    let px = px + offset;
    let py = py + offset;

    let det = ax*by - bx*ay;
    if det == 0 {
        return None;
    }

    let mut a = by * px - bx * py;
    let mut b = ax * py - ay * px;

    // integer division only
    if a % det != 0 || b % det != 0 {
        return None;
    }

    a /= det;
    b /= det;

    if offset!=0 || (a <= 100 || b <= 100) {
        Some((3*a + b) as Tokens)
    } else {
        None
    }
}

fn parse(contents: &str) -> Input {
    let mut ret = Input::new();
    let contents = contents.replace("\n", "");
    let mut scanner = Scanner::new(&contents);
    
    while !scanner.is_done() {
        while let Some(val) = scanner.pop() {
            if *val == '+' {
                break;
            }
        }
        let ax = scanner.try_u32().unwrap() as i128;

        while let Some(val) = scanner.pop() {
            if *val == '+' {
                break;
            }
        }
        let ay = scanner.try_u32().unwrap() as i128;

        while let Some(val) = scanner.pop() {
            if *val == '+' {
                break;
            }
        }
        let bx = scanner.try_u32().unwrap() as i128;

        while let Some(val) = scanner.pop() {
            if *val == '+' {
                break;
            }
        }
        let by = scanner.try_u32().unwrap() as i128;

        while let Some(val) = scanner.pop() {
            if *val == '=' {
                break;
            }
        }
        let px = scanner.try_u32().unwrap() as i128;

        while let Some(val) = scanner.pop() {
            if *val == '=' {
                break;
            }
        }
        let py = scanner.try_u32().unwrap() as i128;

        let rows = vec![ax,bx,px,ay,by,py];
        let width = 3;
        let augmented = Matrix::new(rows, width);

        ret.push(augmented);
    }

    ret
}

