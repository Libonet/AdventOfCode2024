use std::{fs::read_to_string, io};

struct Scanner {
    cursor: usize,
    characters: Vec<char>,
    enabled: bool,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
            enabled: true,
        }
    }

    /// get current cursor
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn r#do(&mut self) {
        self.enabled = true;
    }

    pub fn dont(&mut self) {
        self.enabled = false;
    }

    /// get next char without advancing the cursor
    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    /// return true if cursor is at the end
    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    /// get next char (if available) and advance the cursor
    pub fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;

                Some(character)
            }
            None => None,
        }
    }

    /// return true if chars[cursor] == target and advance the cursor 
    pub fn take(&mut self, target: &char) -> bool {
        match self.characters.get(self.cursor) {
            Some(character) => {
                if target == character {
                    self.cursor += 1;

                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}

fn number(scanner: &mut Scanner) -> Option<i32> {
    let mut num: Option<i32> = None;
    let mut digits = 0;

    while digits < 3 {
        match scanner.peek() {
            Some(chr) => if let Some(digit) = (*chr).to_digit(10) {
                match num {
                    Some(n) => num = Some(n * 10 + digit as i32),
                    None => num = Some(digit as i32),
                }
                digits += 1;
            } else {
                break;
            },
            None => break,
        }
        scanner.pop();
    }

    num
}

fn parse_mul(scanner: &mut Scanner) -> Option<i32> {
    if !scanner.take(&'m') {
        scanner.pop();
        return None
    }
    if !scanner.take(&'u') {
        return None
    }
    if !scanner.take(&'l') {
        return None
    }
    if !scanner.take(&'(') {
        return None
    }

    let left = number(scanner)?;

    if !scanner.take(&',') {
        return None
    }
    
    let right = number(scanner)?;

    if !scanner.take(&')') {
        return None
    }

    Some(left * right)
}

fn part1() -> Result<(), io::Error> {
    let contents = read_to_string("input/day03.txt")?;

    let result: i32 = contents.lines()
        .map(|line| {
            let mut scanner = Scanner::new(line);
            let mut vec: Vec<i32> = Vec::new();
            while !scanner.is_done() {
                if let Some(num) = parse_mul(&mut scanner) {
                    vec.push(num);
                }
            }
            vec
        })
        .reduce(|a,b| [a, b].concat()).unwrap_or(vec![0]).iter()
        .sum();

    println!("part1 result = {result}");

    Ok(())
}

fn parse_do(scanner: &mut Scanner) {
    if !scanner.take(&'d') {
        scanner.pop();
        return;
    }
    if !scanner.take(&'o') {
        scanner.pop();
        return;
    }
    if !scanner.take(&'(') {
        scanner.pop();
        return;
    }
    if !scanner.take(&')') {
        scanner.pop();
        return;
    }
    scanner.r#do();
}

fn parse_dont(scanner: &mut Scanner) -> bool {
    if !scanner.take(&'d') {
        return false;
    }
    if !scanner.take(&'o') {
        return false;
    }
    if !scanner.take(&'n') {
        return false;
    }
    if !scanner.take(&'\'') {
        return false;
    }
    if !scanner.take(&'t') {
        return false;
    }
    if !scanner.take(&'(') {
        return false;
    }
    if !scanner.take(&')') {
        return false;
    }
    scanner.dont();
    true
}

fn parse_text(text: &str) -> i32{
    let mut scanner = Scanner::new(text);
    let mut sum: i32 = 0;

    while !scanner.is_done() {
        if scanner.enabled() {
            if !parse_dont(&mut scanner) {
                if let Some(num) = parse_mul(&mut scanner) {
                    sum += num;
                }
            }
        } else {
            parse_do(&mut scanner);
        }
    }

    sum
}

fn part2() -> Result<(), io::Error> {
    let contents = read_to_string("input/day03.txt")?;

    let text = contents.lines().collect::<Vec<&str>>().join(", ");

    let result: i32 = parse_text(&text);

    println!("part1 result = {result}");

    Ok(())
}

pub fn answer() -> Result<(), io::Error> {
    println!("Part1:");
    part1()?;

    println!("Part2:");
    part2()?;

    Ok(())
}

