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
            let rest: Vec<&str> = ret[1].split_whitespace().collect();
            let mut rest = rest.iter().map(|text| text.parse::<u64>().unwrap());

            let first = rest.next().unwrap();
            let rest: Vec<u64> = rest.collect();

            let results = operate(first, &rest);
            let mut result = 0;
            for val in results {
                if val == target {
                    result = target;
                    break;
                }
            }

            result
        })
        .sum();
        

    println!("result = {result:#?}");

    Ok(())
}

fn operate(acc: u64, values: &[u64]) -> Vec<u64> {
    if values.is_empty() {
        return vec![acc];
    }

    let mut sum = operate(acc + values[0], &values[1..]);
    let mut mul = operate(acc * values[0], &values[1..]);

    sum.append(&mut mul);

    sum
}

fn part2() -> Result<(), io::Error>{
    let contents = read_to_string("input/day07.txt")?; // input/dayxx.txt

    let result: u64 = contents.lines()
        .map(|line| {
            let ret: Vec<&str> = line.split(':').collect();
            let target = ret[0].parse::<u64>().unwrap();

            let rest: Vec<&str> = ret[1].split_whitespace().collect();

            let first = rest[0];
            let rest = &rest[1..];

            let results = operate_str(first, rest);
            let mut result = 0;
            for val in results {
                if val.parse::<u64>().unwrap() == target {
                    result = target;
                    break;
                }
            }

            result
        })
        .sum();
        

    println!("result = {result:#?}");
   
    Ok(())
}

fn operate_str(acc: &str, values: &[&str]) -> Vec<String> {
    if values.is_empty() {
        return vec![acc.to_string()];
    }

    let mut cons = operate_str(&format!("{acc}{}", values[0]), &values[1..]);

    let acc = acc.parse::<u64>().unwrap();
    let val = values[0].parse::<u64>().unwrap();

    let add = (acc + val).to_string();
    let mut sum = operate_str(&add, &values[1..]);
    let mul = (acc * val).to_string();
    let mut mul = operate_str(&mul, &values[1..]);

    sum.append(&mut mul);

    sum.append(&mut cons);

    sum
}

pub fn answer() -> Result<(), io::Error>{
    println!("Part1:");
    part1()?;

    println!("Part2:");
    part2()?;

    Ok(())
}
