use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::{fs::read_to_string, io};
use std::time::Instant;

use crate::matrix::Matrix;
use crate::position::{Pos, UPos};

type Input = Vec<String>;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day21.txt")?; // input/dayxx.txt
    
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
    contents.lines().map(|line| line.to_string()).collect()
}

type Complexity = usize;
fn part1(input: &Input) -> Complexity {
    solve(input, 2)
}

fn solve(input: &Input, robots: usize) -> Complexity {
    let mut sum = 0;
    let mut numpad = Numpad::new();

    let mut keypad = Keypad::new();
    keypad.cache_robots(robots);

    for code in input {
        let start_code = numpad.translate_code((*code).clone());

        let len = keypad.transition_cost(robots, &start_code);

        let num_code = code[..3].parse::<usize>().unwrap();

        let complexity = num_code * len;
        println!("complexity of {code:?} = {len} * {num_code}");

        sum += complexity;
    }

    sum
}

#[derive(Debug)]
struct AnyPad {
    pad: Matrix<char>,
    transitions: Matrix<Vec<String>>,
    memo: HashMap<(usize, char, char), Complexity>,
}

impl AnyPad {
    fn new(pad: Matrix<char>, transitions: Matrix<Vec<String>>) -> Self {
        Self { pad, transitions, memo:HashMap::new() }
    } 
}

struct Numpad(AnyPad);
struct Keypad(AnyPad);

impl Deref for Numpad {
    type Target = AnyPad;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Keypad {
    type Target = AnyPad;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Numpad {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for Keypad {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Numpad {
    fn new() -> Self {
        let width = 3;
        let rows = vec!['7','8','9','4','5','6','1','2','3',' ','0','A'];

        let pad = Matrix::new(rows, width);

        let transitions = Self::create_transitions(&pad);

        //println!("pad:\n{pad}");
        //println!("{:?}", transitions[Numpad::val2pos(&'A').unwrap()]);

        let anypad = AnyPad::new(pad, transitions);
        Self (anypad)
    }

    fn create_transitions(pad: &Matrix<char>) -> Matrix<Vec<String>> {
        let mut filled_vec = Vec::with_capacity(pad.row_count() * pad.width());
        for _i in 0..filled_vec.capacity() {
            filled_vec.push(" ".to_string());
        }

        let mut transitions = Matrix::with_capacity(pad.row_count(), pad.width(), filled_vec);
        for i in 0..pad.row_count() {
            for j in 0..pad.width() {
                for i2 in 0..pad.row_count() {
                    for j2 in 0..pad.width() {
                        let from_val = pad[UPos(i,j)];
                        let to_val = pad[UPos(i2,j2)];
                        let from = Self::val2pos(&from_val);
                        let to = Self::val2pos(&to_val);
                        if from.is_none() || to.is_none() {
                            continue;
                        }

                        let from = from.unwrap();
                        let to = to.unwrap();
                        let trans = Self::offset2transition(&from, &to);
                        transitions[from][Self::val2index(&to_val).unwrap()] = trans;
                    }
                }
            }
        }
        transitions
    }

    fn offset2transition(start: &UPos, end: &UPos) -> String {
        let from_pos: Pos = (*start).into();
        let to_pos: Pos = (*end).into();
        let offset = to_pos-from_pos;
        let Pos(x,y) = offset;

        // protect from edge cases:
        if let UPos(3,_h) = *start {
            // we're going to pass over the gap
            if let UPos(_v,0) = *end {
                let mut ret = String::new();
                // go up for safety
                let vert = "^".repeat(x.unsigned_abs() as usize);
                ret.push_str(&vert);
                let hori = "<".repeat(y.unsigned_abs() as usize);
                ret.push_str(&hori);
                ret.push('A');

                return ret;
            }
        }
        if let UPos(_v,0) = *start {
            // we're going to pass over the gap
            if let UPos(3,_h) = *end {
                let mut ret = String::new();
                // go right for safety
                let hori = ">".repeat(y.unsigned_abs() as usize);
                ret.push_str(&hori);
                let vert = "v".repeat(x.unsigned_abs() as usize);
                ret.push_str(&vert);
                ret.push('A');

                return ret;
            }
        }

        // left is the worst because it leaves you
        // far away from the A button. Because of that
        // we do it first, and end in a button closer to A
        let priority = |val| {
            match val {
                "<" => 3,
                "v" => 2,
                "^" => 1,
                ">" => 0,
                _ => -1,
            }
        };

        let vert = match x.signum() {
            -1 => "^".to_string(),
            1 => "v".to_string(),
            0 => "".to_string(),
            _ => unreachable!(),
        };
        let mut vert = (vert.repeat(x.unsigned_abs() as usize), priority(&vert));

        let hori = match y.signum() {
            -1 => "<".to_string(),
            1 => ">".to_string(),
            0 => "".to_string(),
            _ => unreachable!(),
        };
        let mut hori = (hori.repeat(y.unsigned_abs() as usize), priority(&hori));

        // compare priorities
        if hori.1 > vert.1 {
            hori.0.push_str(&vert.0);
            hori.0.push('A');
            hori.0
        } else {
            vert.0.push_str(&hori.0);
            vert.0.push('A');
            vert.0
        }
    }

    fn val2pos(val: &char) -> Option<UPos> {
        match val {
            '7'..='9' => Some(UPos(0, (val.to_digit(10).unwrap()-7) as usize)),
            '4'..='6' => Some(UPos(1, (val.to_digit(10).unwrap()-4) as usize)),
            '1'..='3' => Some(UPos(2, (val.to_digit(10).unwrap()-1) as usize)),
            '0' => Some(UPos(3,1)),
            'A' => Some(UPos(3,2)),
            _ => None,
        }
    }

    fn val2index(val: &char) -> Option<usize> {
        match val {
            '0'..='9' => Some(val.to_digit(10).unwrap() as usize),
            'A' => Some(10),
            _ => None,
        }
    }

    fn get_transition(&self, from: char, to: char) -> &str {
        let from_pos = Self::val2pos(&from).unwrap();
        let to_index = Self::val2index(&to).unwrap();

        &self.0.transitions[from_pos][to_index]
    }

    fn translate_code(&mut self, code: String) -> String {
        //println!("code: {code:?}");

        let mut translation = String::new();
        let mut from = 'A';
        for val in code.chars() {
            //self.print(&from);
            translation.push_str(self.get_transition(from, val));
            from = val;
        }
        //println!("translation: {translation:?}");

        translation
    } 

    fn print(&self, curr_val: &char) {
        for (i, val)  in self.pad.iter().enumerate() {
            if i % self.pad.width() == 0 {
                println!();
            }
            if *val == *curr_val {
                print!("\x1b[93m {val} \x1b[0m");
            } else {
                print!(" {val} ");
            }
        }
        println!();
    }
}

impl Keypad {
    fn new() -> Self {
        let width = 3;
        let rows = vec![' ','^','A','<','v','>'];

        let pad = Matrix::new(rows, width);

        let transitions = Self::create_transitions(&pad);

        //println!("pad:\n{pad}");
        //println!("{:?}", transitions[Numpad::val2pos(&'A').unwrap()]);

        let anypad = AnyPad::new(pad, transitions);
        Self (anypad)
    }

    fn create_transitions(pad: &Matrix<char>) -> Matrix<Vec<String>> {
        let mut filled_vec = Vec::with_capacity(pad.row_count() * pad.width());
        for _i in 0..filled_vec.capacity() {
            filled_vec.push(" ".to_string());
        }

        let mut transitions = Matrix::with_capacity(pad.row_count(), pad.width(), filled_vec);
        for i in 0..pad.row_count() {
            for j in 0..pad.width() {
                for i2 in 0..pad.row_count() {
                    for j2 in 0..pad.width() {
                        let from_val = pad[UPos(i,j)];
                        let to_val = pad[UPos(i2,j2)];
                        let from = Self::val2pos(&from_val);
                        let to = Self::val2pos(&to_val);
                        if from.is_none() || to.is_none() {
                            continue;
                        }

                        let from = from.unwrap();
                        let to = to.unwrap();
                        let trans = Self::offset2transition(&from, &to);
                        transitions[from][Self::val2index(&to_val).unwrap()] = trans;
                    }
                }
            }
        }
        transitions
    }

    fn offset2transition(start: &UPos, end: &UPos) -> String {
        let from_pos: Pos = (*start).into();
        let to_pos: Pos = (*end).into();
        let offset = to_pos-from_pos;
        let Pos(x,y) = offset;

        // protect from edge cases:
        if let UPos(0,_h) = *start {
            // we're going to pass over the gap
            // and end in '<'
            if let UPos(1,0) = *end {
                let mut ret = String::new();
                // go down for safety
                let vert = "v".repeat(x.unsigned_abs() as usize);
                ret.push_str(&vert);
                let hori = "<".repeat(y.unsigned_abs() as usize);
                ret.push_str(&hori);
                ret.push('A');

                return ret;
            }
        }
        if let UPos(1,0) = *start {
            // we're going to pass over the gap
            // starting from '<'
            if let UPos(0,_h) = *end {
                let mut ret = String::new();
                // go right for safety
                let hori = ">".repeat(y.unsigned_abs() as usize);
                ret.push_str(&hori);
                let vert = "^".repeat(x.unsigned_abs() as usize);
                ret.push_str(&vert);
                ret.push('A');

                return ret;
            }
        }

        let priority = |val| {
            match val {
                "<" => 3,
                "v" => 2,
                "^" => 1,
                ">" => 0,
                _ => -1,
            }
        };

        let vert = match x.signum() {
            -1 => "^".to_string(),
            1 => "v".to_string(),
            0 => "".to_string(),
            _ => unreachable!(),
        };
        let mut vert = (vert.repeat(x.unsigned_abs() as usize), priority(&vert));

        let hori = match y.signum() {
            -1 => "<".to_string(),
            1 => ">".to_string(),
            0 => "".to_string(),
            _ => unreachable!(),
        };
        let mut hori = (hori.repeat(y.unsigned_abs() as usize), priority(&hori));

        // compare priorities
        if hori.1 > vert.1 {
            hori.0.push_str(&vert.0);
            hori.0.push('A');
            hori.0
        } else {
            vert.0.push_str(&hori.0);
            vert.0.push('A');
            vert.0
        }
    }

    fn val2pos(val: &char) -> Option<UPos> {
        match val {
            '^' => Some(UPos(0,1)),
            'A' => Some(UPos(0,2)),
            '<' => Some(UPos(1,0)),
            'v' => Some(UPos(1,1)),
            '>' => Some(UPos(1,2)),
            _ => None,
        }
    }

    fn val2index(val: &char) -> Option<usize> {
        match val {
            '^' => Some(0),
            '<' => Some(1),
            '>' => Some(2),
            'v' => Some(3),
            'A' => Some(4),
            _ => None,
        }
    }

    fn get_transition(&self, from: char, to: char) -> &str {
        let from_pos = Self::val2pos(&from).unwrap();
        let to_index = Self::val2index(&to).unwrap();

        &self.transitions[from_pos][to_index]
    }

    fn cache_robots(&mut self, n_robots: usize) {
        self.memo = HashMap::new();

        for robot in 1..=n_robots {
            println!("caching robot {robot}");
            self.cache_robot(robot);
        }
    }

    fn cache_robot(&mut self, robot: usize) {
        let mut insert_list = Vec::new();
        for &key_start in self.pad.iter() {
            for &key_end in self.pad.iter() {
                if key_start == ' ' || key_end == ' ' {
                    continue;
                }
                println!("keys: {key_start}, {key_end}");
                let transition = self.get_transition(key_start, key_end);
                println!("transition: {transition:?}");

                let cost = self.transition_cost(robot - 1, transition);
                println!("cost = {cost}");

                insert_list.push((key_start, key_end, cost));
            }
        }
        for (key_start, key_end, cost) in insert_list {
            self.memo.insert(
                (robot, key_start, key_end),
                cost);
        }
    }

    fn transition_cost(
        &self,
        robots: usize,
        transition: &str,
    ) -> Complexity {
        let path: Vec<char> = format!("A{transition}").chars().collect();
        let mut sum = 0;
        for (end, end_key) in path.iter().enumerate().skip(1) {
            let start_key = path[end-1];
            sum += self.key_path_cost(robots, start_key, *end_key);
        }

        sum
    }

    fn key_path_cost(
        &self,
        robots: usize,
        start: char,
        end: char,
    ) -> Complexity {
        if robots == 0 {
            1
        } else {
            *self.memo
                .get(&(robots, start, end))
                .unwrap_or_else(|| panic!("invalid key '{} {} {}'", robots, start, end))
        }
    }

    //fn print(&self, curr_val: &char) {
    //    for (i, val)  in self.pad.iter().enumerate() {
    //        if i % self.pad.width() == 0 {
    //            println!();
    //        }
    //        if *val == *curr_val {
    //            print!("\x1b[93m {val} \x1b[0m");
    //        } else {
    //            print!(" {val} ");
    //        }
    //    }
    //    println!();
    //}
}

fn part2(input: &Input) -> Complexity {
    solve(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypad() {
        let mut keypad = Keypad::new();
        keypad.cache_robots(2);

        let mut numpad = Numpad::new();

        let code = "029A".to_string();
        let code = numpad.translate_code(code);

        let result = keypad.transition_cost(1, &code);

        assert_eq!(result, "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
    }

    #[test]
    fn test_keypad_2_robots() {
        let robot_count = 2;

        let mut numpad = Numpad::new();
        let mut keypad = Keypad::new();
        keypad.cache_robots(robot_count);

        let code = "456A".to_string();
        let code = numpad.translate_code(code);
        
        let result = keypad.transition_cost(robot_count, &code);

        let expected = "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A";

        assert_eq!(
            result,
            expected.len()
        );
    }

    #[test]
    fn test_part1() {
        let contents = "\
029A
980A
179A
456A
379A";

        let input = parse(contents.to_string());

        let result = part1(&input);

        assert_eq!(result, 126384);
    }
}
