use std::{fs::read_to_string, io};

/*
#[derive(Debug)]
struct Node {
    val: i32,
    sum: SubTree,
    mul: SubTree,
}

impl Node {
    fn new() -> Self {
        Self {
            val: 0,
            sum: SubTree(None),
            mul: SubTree(None),
        }
    }
}

#[derive(Debug)]
struct SubTree (Option<Box<Node>>);

impl SubTree {
    fn new() -> Self {
        Self ( Some(Box::new(Node::new())) )
    }

    fn insert_list(&mut self, values: &[i32]) -> Option<Vec<i32>> {
        if values.is_empty() {
            return None;
        }


    }
}

#[derive(Debug)]
struct OpTree {
    root: SubTree,
}

impl OpTree {
    fn new() -> Self {
        Self { root: SubTree::new() }
    }

    fn insert_list(&mut self, values: &[i32]) -> Option<Vec<i32>> {
        if values.is_empty() {
            return None;
        }

        self.root.insert_list(values);
    }
}
*/

fn part1() -> Result<(), io::Error>{
    let contents = read_to_string("input/day07.txt")?; // input/dayxx.txt

    let result: u64 = contents.lines()
        .map(|line| {
            let ret: Vec<&str> = line.split(':').collect();
            let target = ret[0].parse::<u64>().unwrap();

            let rest: Vec<u64> = ret[1]
                .split_whitespace()
                .rev()
                .map(|text| text.parse::<u64>().unwrap())
                .collect();

            if better_operate(target, &rest) {
                target
            } else {
                0
            }
        })
        .sum();

    println!("result = {result:#?}");

    Ok(())
}

/*
fn operate(acc: u64, values: &[u64]) -> Vec<u64> {
    if values.is_empty() {
        return vec![acc];
    }

    let mut sum = operate(acc + values[0], &values[1..]);
    let mut mul = operate(acc * values[0], &values[1..]);

    sum.append(&mut mul);

    sum
}
*/

fn better_operate(acc: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        return false;
    }
    if acc == values[0] {
        return true;
    }

    let mut mul = false;
    if acc % values[0] == 0 {
        mul = better_operate(acc / values[0], &values[1..]);
    }

    let sum = better_operate(acc - values[0], &values[1..]);

    sum || mul
}

fn part2() -> Result<(), io::Error>{
    let contents = read_to_string("input/day07.txt")?; // input/dayxx.txt

    let result: u64 = contents.lines()
        .map(|line| {
            let ret: Vec<&str> = line.split(':').collect();
            let target = ret[0].parse::<u64>().unwrap();

            let rest: Vec<u64> = ret[1]
                .split_whitespace()
                .rev()
                .map(|text| text.parse::<u64>().unwrap())
                .collect();


            if better_operate_cons(target, &rest) {
                target
            } else {
                0
            }
        })
        .sum();

    println!("result = {result:#?}");

    Ok(())
}

/*
fn operate_cons(acc: &str, values: &[&str]) -> Vec<String> {
    if values.is_empty() {
        return vec![acc.to_string()];
    }

    let mut cons = operate_cons(&format!("{acc}{}", values[0]), &values[1..]);

    let acc = acc.parse::<u64>().unwrap();
    let val = values[0].parse::<u64>().unwrap();

    let add = (acc + val).to_string();
    let mut sum = operate_cons(&add, &values[1..]);
    let mul = (acc * val).to_string();
    let mut mul = operate_cons(&mul, &values[1..]);

    sum.append(&mut mul);

    sum.append(&mut cons);

    sum
}
*/

// go backwards from target to 0
fn better_operate_cons(acc: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        return false;
    }
    if acc == values[0] {
        return true;
    }

    let mut mul = false;
    if acc % values[0] == 0 {
        mul = better_operate_cons(acc / values[0], &values[1..]);
    }
    
    let new_acc = acc - values[0];
    let sum = better_operate_cons(new_acc, &values[1..]);

    let mut cons = false;
    let tens = 10u64.pow(values[0].ilog10() + 1);
    if new_acc % tens == 0 {
        cons = better_operate_cons(new_acc / tens, &values[1..]);
    }

    sum || mul || cons
}

pub fn answer() -> Result<(), io::Error>{
    println!("Part1:");
    part1()?;

    println!("Part2:");
    part2()?;

    Ok(())
}
