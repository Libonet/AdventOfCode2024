use std::{cmp::Reverse, collections::BinaryHeap, fs::read_to_string, io};

use crate::scanner::Scanner;

type Register = u64;
type Op = u8;

#[derive(Debug, Clone, Default)]
struct Program {
    a: Register,
    b: Register,
    c: Register,
    ops: Vec<Op>,
    ip: usize,
}

impl Program {
    fn new(a: Register, b: Register, c: Register, ops: Vec<Op>) -> Self {
        Self { a, b, c, ops, ip: 0 }
    }

    fn run(&mut self) -> Vec<Op> {
        let mut output: Vec<Op> = Vec::new();

        while self.ip < self.ops.len() {
            let res = self.execute();
            if let Some(val) = res {
                output.push(val);
            }
        }

        output
    }

    fn execute(&mut self) -> Option<Op> {
        let code = self.ops[self.ip];
        let operand = self.ops[self.ip+1];

        match code {
            0 => {
                // adv: divides A by 2^combo, then writes to A
                let combo = self.get_combo(operand);
                let den = 2_u64.pow(combo as u32);

                self.a /= den;
            },
            1 => {
                // bxl: bitwise XOR of B and literal operand, then writes to B
                self.b ^= operand as Register;
            },
            2 => {
                // bst: combo modulo 8, then writes to B
                let combo = self.get_combo(operand);
                self.b = combo % 8;
            },
            3 => {
                // jnz: jump to literal operand if A not 0
                if self.a != 0 {
                    self.ip = operand as usize;
                    return None;
                }
            },
            4 => {
                // bxc: bitwise XOR of B and C, then writes to B
                self.b ^= self.c;
            },
            5 => {
                // out: combo operand modulo 8, then outputs that value
                let combo = self.get_combo(operand);

                let result = (combo % 8) as u8;
                self.ip += 2;
                return Some(result);
            },
            6 => {
                // bdv: like adv, but writes to B
                let combo = self.get_combo(operand);
                let den = 2_u64.pow(combo as u32);

                self.b = self.a / den;
            },
            7 => {
                // cdv: like adv, but writes to C
                let combo = self.get_combo(operand);
                let den = 2_u64.pow(combo as u32);

                self.c = self.a / den;
            },
            _ => unreachable!(),
        }

        self.ip += 2;
        None
    }

    fn get_combo(&self, operand: Op) -> Register {
        match operand {
            val @ (0..=3) => val.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

type Input = Program;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day17.txt")?; // input/dayxx.txt

    let input = parse(contents);

    // println!("{input:?}");
    
    println!("Part1:");
    let part1_res = part1(&input);

    let mut iter = part1_res.iter();
    let first = iter.next().unwrap();
    print!("{first}");
    for next in iter {
        print!(",{next}");
    }
    println!();

    println!("Part2:");
    if let Some(part2_res) = part2(&input) {
        println!("result = {part2_res}");
    } else {
        println!("No result for part 2");
    }

    Ok(())
}

fn parse(contents: String) -> Input {
    let mut scanner = Scanner::new(&contents);

    while *scanner.pop().unwrap() != ':' {}
    scanner.take(&' ');
    let a = scanner.try_u32().unwrap() as u64;

    while *scanner.pop().unwrap() != ':' {}
    scanner.take(&' ');
    let b = scanner.try_u32().unwrap() as u64;

    while *scanner.pop().unwrap() != ':' {}
    scanner.take(&' ');
    let c = scanner.try_u32().unwrap() as u64;

    while *scanner.pop().unwrap() != ':' {}
    scanner.take(&' ');

    let mut ops = Vec::new();
    while !scanner.is_done() {
        let num = scanner.try_u32();
        if let Some(val) = num {
            ops.push(val as u8);
            // pop the ,
            scanner.pop();
        } else {
            // pop the last '\n'
            scanner.pop();
        }
    }

    Program { a, b, c, ops, ip:0 }
}

fn part1(input: &Input) -> Vec<Op> {
    let mut program = input.clone();

    program.run()
}

fn part2(input: &Input) -> Option<Register> {
    let mut program = (*input).clone();

    // Program:
    // (2,4): B = A % 8 (get last 3 bits from A),
    // (1,5): B = B ^ 5,
    // (7,5): C = A / 2.pow(B)
    // (4,3): B = B ^ C,
    // (1,6): B = B ^ 6,
    // (0,3): A = A / 2^3 (shifts the bits 3 spaces right),
    // (5,5): Print (B % 8) (print last 3 bits from B),
    // (3,0): Jumps to start if A is not 0,
    //
    // Output = ((((A % 8) ^ 101) ^ (A >> ((A % 8) ^ 5))) ^ 110) % 8
    // A = A >> 3
    //
    // Each value of the output depends only on all the possible last 3 bits of A,
    // because B and C depend on the value of A
    //
    //println!("PROGRAM!!!");
    //println!("{:?}", program.ops);
    
    // To get each position X of the program as output,
    // we shift 3 times left and look from there for the next
    let prog_len = program.ops.len();

    let mut heap = BinaryHeap::new();

    for offset in 0..8 {
        heap.push(Reverse(offset));
    }

    while let Some(Reverse(val)) = heap.pop() {
        program.a = val;
        program.b = 0;
        program.c = 0;
        program.ip = 0;

        let output = program.run();

        if output == program.ops {
            return Some(val);
        }

        let output_len = output.len();
        if output == program.ops[prog_len - output_len ..] {
            let new_start = val << 3;
            for offset in 0..8 {
                heap.push(Reverse(new_start + offset));
            }
        }
    }

    None
}
