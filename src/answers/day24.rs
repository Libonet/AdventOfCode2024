use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::{fs::read_to_string, io};
use std::time::Instant;

use crate::scanner::Scanner;

type Wire = String;
type Wires = HashMap<Wire, bool>;

type Gates = Vec<Gate>;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Gate {
    a: Wire,
    op: Ops,
    b: Wire,
    target: Wire,
}

impl Gate {
    fn new(a: Wire, op: Ops, b: Wire, target: Wire) -> Self {
        Self { a, op, b, target }
    }

    fn execute(&self, wires: &mut Wires, lut: &HashMap<String, Gate>) {
        let a = match wires.get(&self.a) {
            Some(v) => *v,
            None => {
                let gate = &lut[&self.a];
                gate.execute(wires, lut);
                wires[&self.a]
            },
        };
        let b = match wires.get(&self.b) {
            Some(v) => *v,
            None => {
                let gate = &lut[&self.b];
                gate.execute(wires, lut);
                wires[&self.b]
            },
        };
        println!("executing {:?}={a:?} {:?} {:?}={b:?} into {:?}",
            self.a, self.op, self.b, self.target);
        wires.insert(self.target.clone(), self.op.calculate(a, b));
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Ops {
    And,
    Or,
    Xor,
}

impl Ops {
    fn calculate(&self, a: bool, b: bool) -> bool {
        match self {
            Ops::And => a & b,
            Ops::Or => a | b,
            Ops::Xor => a ^ b,
        }
    }
}

impl FromStr for Ops {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Ops::And),
            "OR" => Ok(Ops::Or),
            "XOR" => Ok(Ops::Xor),
            _ => Err("Invalid string. try 'AND', 'OR', or 'XOR'"),
        }
    }
}

type Input = (Wires, Gates);

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day24.txt")?; // input/dayxx.txt
    
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
    let (wires_str, gates_str) = contents.split_once("\n\n").unwrap();

    let mut wires = Wires::new();
    for line in wires_str.lines() {
        let name = &line[0..3];
        let val = &line[5..6] == "1";

        *wires.entry(name.to_string()).or_default() = val;
    }

    let mut gates = Gates::new();
    let mut scanner = Scanner::new(gates_str);
    while !scanner.is_done() {
        let a = scanner.popn(3).unwrap();
        scanner.pop();
        let op = match scanner.popn(3) {
            Some(str) => {
                if &str == "OR " {
                    Ops::Or
                } else {
                    scanner.pop();
                    Ops::from_str(&str).expect("should be valid Ops in input")
                }
            },
            None => unreachable!("Input is well constructed"),
        };
        let b = scanner.popn(3).unwrap();
        scanner.pop();
        scanner.pop();
        scanner.pop();
        scanner.pop();
        let target = scanner.popn(3).unwrap();
        scanner.pop();

        let gate = Gate::new(a,op,b,target);
        gates.push(gate);
    }

    (wires, gates)
}

type Decimal = u64;

fn part1(input: &Input) -> Decimal {
    let (mut wires, gates) = (*input).clone();
    
    let mut gate_lut = HashMap::new();
    let mut ready = VecDeque::new();
    for gate in gates {
        if gate.target.starts_with("z") {
            ready.push_back(gate.target.clone());
        }
        gate_lut.insert(gate.target.clone(), gate);
    }


    while !ready.is_empty() {
        let wire = ready.pop_front().unwrap();
        let gate = &gate_lut[&wire];

        println!("starting gate of {wire:?}");

        gate.execute(&mut wires, &gate_lut);
    }

    let mut outputs = Vec::new();
    for wire in wires.keys() {
        if wire.starts_with("z") {
            outputs.push(wire);
        }
    }

    outputs.sort();

    let mut iter = outputs.into_iter().rev();
    let mut result = 0;
    let v = wires[iter.next().unwrap()];
    if v {
        result += 1;
    }
    for z in iter {
        result <<= 1;
        let v = wires[z];
        if v {
            result += 1;
        }
    }

    result
}

fn part2(input: &Input) -> i32 {
    let result = 0; // so rust shuts up

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj".to_string();

        let input = parse(contents);

        let result = part1(&input);

        assert_eq!(result, 2024);
    }
}
