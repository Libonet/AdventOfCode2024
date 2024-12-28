use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
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
        //println!("executing {:?}={a:?} {:?} {:?}={b:?} into {:?}",
            //self.a, self.op, self.b, self.target);
        wires.insert(self.target.clone(), self.op.calculate(a, b));
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} -> {}", self.a, self.op, self.b, self.target)
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

impl Display for Ops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ops::And => write!(f, "AND"),
            Ops::Or => write!(f, "OR"),
            Ops::Xor => write!(f, "XOR"),
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

        gate.execute(&mut wires, &gate_lut);
    }

    let mut outputs = Vec::new();
    for wire in wires.keys() {
        if wire.starts_with("z") {
            outputs.push(wire);
        }
    }

    outputs.sort();

    into_decimal(&outputs, &wires)
}

fn into_decimal(outputs: &[&String], wires: &Wires) -> Decimal {
    let mut iter = outputs.iter().rev();
    let mut result = 0;
    let v = wires[*iter.next().unwrap()];
    if v {
        result += 1;
    }
    for z in iter {
        result <<= 1;
        let v = wires[*z];
        if v {
            result += 1;
        }
    }

    result
}

fn part2(input: &Input) -> Vec<String> {
    let (mut wires, mut gates) = (*input).clone();

    //let mut list: Vec<_> = wires.iter().collect();
    //list.sort();

    //println!("List:");
    //println!("{list:?}");
    
    let mut alias = HashMap::new();
    let mut reverse_alias = HashMap::new();
    for gate in gates.iter() {
        if gate.target.starts_with("z") {
            // dont want to rename outputs
            continue;
        }
        if gate.a.starts_with("x") && gate.b.starts_with("y") {
            let op = gate.op.to_string();
            let num: String = gate.a.chars().skip(1).collect();
            let new_name = format!("{op}xy{num}");
            if &new_name == "ANDxy00" {
                alias.insert(gate.target.clone(), "CARRY00".to_string());
                reverse_alias.insert("CARRY00".to_string(), gate.target.clone());
            } else {
                reverse_alias.insert(new_name.clone(), gate.target.clone());
                alias.insert(gate.target.clone(), new_name);
            }
        } else if gate.b.starts_with("x") && gate.a.starts_with("y") {
            let op = gate.op.to_string();
            let num: String = gate.a.chars().skip(1).collect();
            let new_name = format!("{op}yx{num}");
            reverse_alias.insert(new_name.clone(), gate.target.clone());
            alias.insert(gate.target.clone(), new_name);
        } else {
            match gate.target.as_str() {
                "qnf" => {
                    alias.insert(gate.target.clone(), "CARRY01".to_string());
                    reverse_alias.insert("CARRY01".to_string(), gate.target.clone());
                },
                "vnm" => {
                    alias.insert(gate.target.clone(), "CARRY02".to_string());
                    reverse_alias.insert("CARRY02".to_string(), gate.target.clone());
                },
                "jdk" => {
                    alias.insert(gate.target.clone(), "CARRY03".to_string());
                    reverse_alias.insert("CARRY03".to_string(), gate.target.clone());
                },
                "jtm" => {
                    alias.insert(gate.target.clone(), "CARRY04".to_string());
                    reverse_alias.insert("CARRY04".to_string(), gate.target.clone());
                },
                _ => (),
            }
        }
    }

    // apply alias
    for gate in gates.iter_mut() {
        if let Some(alias) = alias.get(&gate.target) { gate.target = alias.to_string() }

        if let Some(alias) = alias.get(&gate.a) { gate.a = alias.to_string() }

        if let Some(alias) = alias.get(&gate.b) { gate.b = alias.to_string() }
    } 

    let mut x_num: i64 = 20_000_000_000_000;
    let mut y_num: i64 = 5_000_000_000_000;
    for a in 0..=4 {
        for b in 0..=9 {
            if a == 4 && b == 5 {
                break;
            }
            let x = format!("x{a}{b}");
            let y = format!("y{a}{b}");
            let x_bit = x_num & 1;
            let y_bit = y_num & 1;
            wires.insert(x, x_bit==1);
            wires.insert(y, y_bit==1);
            x_num >>= 1;
            y_num >>= 1;
        }
    }

    let mut x_list = Vec::new();
    let mut y_list = Vec::new();
    for wire in wires.keys() {
        if wire.starts_with("x") {
            x_list.push(wire);
        }
        if wire.starts_with("y") {
            y_list.push(wire);
        }
    }
    x_list.sort();
    y_list.sort();
    let expected = into_decimal(&x_list, &wires) + into_decimal(&y_list, &wires);


    let mut gate_lut = HashMap::new();
    for gate in gates {
        gate_lut.insert(gate.target.clone(), gate);
    }

    let mut swaps = vec![
        "fkb".to_string(),"z16".to_string(),
        reverse_alias["XORxy21"].clone(),reverse_alias["ANDyx21"].clone(),
        "rdn".to_string(),"z31".to_string(),
        "rrn".to_string(),"z37".to_string(),
    ];

    // mistake: bit 16
    // carry of 16 was swapped with z16
    let mut result_gate = gate_lut["fkb"].clone();
    let mut carry_gate = gate_lut["z16"].clone();

    carry_gate.target = "fkb".to_string();
    result_gate.target = "z16".to_string();

    gate_lut.insert("fkb".to_string(), carry_gate);
    gate_lut.insert("z16".to_string(), result_gate);

    println!("carry_gate 16: {}", &gate_lut["fkb"]);
    println!("result_gate 16: {}", &gate_lut["z16"]);

    // mistake: bit 21
    // XOR and AND are used in the wrong places
    let mut xor_gate = gate_lut["XORxy21"].clone();
    let mut and_gate = gate_lut["ANDyx21"].clone();

    xor_gate.target = "ANDyx21".to_string();
    and_gate.target = "XORxy21".to_string();

    gate_lut.insert("ANDyx21".to_string(), xor_gate);
    gate_lut.insert("XORxy21".to_string(), and_gate);

    // mistake: bit 31
    // carry of 31 was swapped with z31
    let mut result_gate = gate_lut["rdn"].clone();
    let mut carry_gate = gate_lut["z31"].clone();

    carry_gate.target = "rdn".to_string();
    result_gate.target = "z31".to_string();

    gate_lut.insert("rdn".to_string(), carry_gate);
    gate_lut.insert("z31".to_string(), result_gate);

    // mistake: bit 37
    // AND of 37 was swapped with z37
    let mut result_gate = gate_lut["rrn"].clone();
    let mut carry_gate = gate_lut["z37"].clone();

    carry_gate.target = "rrn".to_string();
    result_gate.target = "z37".to_string();

    gate_lut.insert("rrn".to_string(), carry_gate);
    gate_lut.insert("z37".to_string(), result_gate);

    let mut sorted: Vec<Gate> = gate_lut.values().cloned().collect();
    
    sorted.sort_by(|a,b| {
        if let Some(i) = a.a.find(|c: char| c.is_ascii_digit()) {
            if let Some(j) = b.a.find(|c: char| c.is_ascii_digit()) {
                a.a[i..].cmp(&b.a[j..])
            } else if let Some(j) = b.b.find(|c: char| c.is_ascii_digit()) {
                a.a[i..].cmp(&b.b[j..])
            } else if let Some(j) = b.target.find(|c: char| c.is_ascii_digit()) {
                a.a[i..].cmp(&b.target[j..])
            } else {
                Ordering::Less
            }
        } else if let Some(i) = a.b.find(|c: char| c.is_ascii_digit()) {
            if let Some(j) = b.a.find(|c: char| c.is_ascii_digit()) {
                a.b[i..].cmp(&b.a[j..])
            } else if let Some(j) = b.b.find(|c: char| c.is_ascii_digit()) {
                a.b[i..].cmp(&b.b[j..])
            } else if let Some(j) = b.target.find(|c: char| c.is_ascii_digit()) {
                a.b[i..].cmp(&b.target[j..])
            } else {
                Ordering::Less
            }
        } else if let Some(i) = a.target.find(|c: char| c.is_ascii_digit()) {
            if let Some(j) = b.a.find(|c: char| c.is_ascii_digit()) {
                a.target[i..].cmp(&b.a[j..])
            } else if let Some(j) = b.b.find(|c: char| c.is_ascii_digit()) {
                a.target[i..].cmp(&b.b[j..])
            } else if let Some(j) = b.target.find(|c: char| c.is_ascii_digit()) {
                a.target[i..].cmp(&b.target[j..])
            } else {
                Ordering::Less
            }
        } else if let Some(_j) = b.a.find(|c: char| c.is_ascii_digit()) {
            Ordering::Greater
        } else if let Some(_j) = b.b.find(|c: char| c.is_ascii_digit()) {
            Ordering::Greater
        } else if let Some(_j) = b.target.find(|c: char| c.is_ascii_digit()) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    for gate in sorted {
        println!("Gate: {gate}");
    }

    let result = part1(&(wires.clone(), gate_lut.values().cloned().collect()));
    println!("result_decimal   = {result}");
    println!("expected_decimal = {expected}");
    println!("result  : {result:046b}");
    println!("expected: {expected:046b}");

    let mut exp = expected;
    let mut res = result;
    for error_pos in 0..45 {
        let bit_exp = exp & 1;
        let bit_res = res & 1;

        if bit_exp != bit_res {
            println!("error bit is in position {error_pos}");
            break;
        }
        res >>= 1;
        exp >>= 1;
    }

    swaps.sort();
    swaps
}

fn test_adder(num: &str, input: &Input) {
    println!("testing {num}");

    let (mut wires, gates) = (*input).clone();
    let mut gate_lut = HashMap::new();
    for gate in gates {
        gate_lut.insert(gate.target.clone(), gate);
    }

    println!("0,0");
    *wires.entry(format!("x{num}")).or_default() = false;
    *wires.entry(format!("y{num}")).or_default() = false;

    gate_lut[&format!("z{num}")].execute(&mut wires, &gate_lut);
    println!("z = {}", wires[&format!("z{num}")]);

    let (mut wires, gates) = (*input).clone();
    let mut gate_lut = HashMap::new();
    for gate in gates {
        gate_lut.insert(gate.target.clone(), gate);
    }

    println!("1,0");
    *wires.entry(format!("x{num}")).or_default() = true;
    *wires.entry(format!("y{num}")).or_default() = false;

    gate_lut[&format!("z{num}")].execute(&mut wires, &gate_lut);
    println!("z = {}", wires[&format!("z{num}")]);

    let (mut wires, gates) = (*input).clone();
    let mut gate_lut = HashMap::new();
    for gate in gates {
        gate_lut.insert(gate.target.clone(), gate);
    }

    println!("0,1");
    *wires.entry(format!("x{num}")).or_default() = false;
    *wires.entry(format!("y{num}")).or_default() = true;

    gate_lut[&format!("z{num}")].execute(&mut wires, &gate_lut);
    println!("z = {}", wires[&format!("z{num}")]);

    let (mut wires, gates) = (*input).clone();
    let mut gate_lut = HashMap::new();
    for gate in gates {
        gate_lut.insert(gate.target.clone(), gate);
    }

    println!("1,1");
    *wires.entry(format!("x{num}")).or_default() = true;
    *wires.entry(format!("y{num}")).or_default() = true;

    gate_lut[&format!("z{num}")].execute(&mut wires, &gate_lut);
    println!("z = {}", wires[&format!("z{num}")]);
}

fn print_dependencies(node: &String, gate_lut: &HashMap<String, Gate>) {
    let mut q = VecDeque::new();
    q.push_back((0, node));
    println!("Dependencies of {node:?}");

    let mut prev_lvl = 0;
    while !q.is_empty() {
        let (lvl, node) = q.pop_front().unwrap();

        if prev_lvl != lvl {
            prev_lvl = lvl;
            println!();
        }

        let gate = gate_lut.get(node);

        match gate {
            Some(gate) => {
                q.push_back((lvl+1, &gate.a));
                q.push_back((lvl+1, &gate.b));
                print!("{node}: {} {:?} {}; ",&gate.a, gate.op,&gate.b);
            },
            None => print!("{node},"),
        }
    }
    println!();
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
