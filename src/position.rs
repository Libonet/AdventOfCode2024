use std::ops::{Mul, Add, Sub};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Pos (pub i32, pub i32);

impl Pos {
    pub fn new(x:i32, y:i32) -> Self {
        Self (x,y)
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Self (self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<i32> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i32) -> Self::Output {
        Self (self.0 * rhs, self.1 * rhs)
    }
}

pub struct PosIter {
    pos: Pos,
    width: usize,
}

impl PosIter {
    pub fn new(width: usize) -> Self {
        Self {
            pos: Pos::new(0, 0),
            width
        }
    }
}

impl Iterator for PosIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.1 < self.width as i32 {
            let ret = Some(self.pos);

            self.pos.1 += 1;
            ret
        } else {
            self.pos.1 = 0;
            self.pos.0 += 1;

            let ret = Some(self.pos);

            self.pos.1 += 1;
            ret
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct UPos(pub usize, pub usize);

impl UPos {
    pub fn new(x:usize, y:usize) -> Self {
        Self (x,y)
    }
}

impl Sub for UPos {
    type Output = UPos;

    fn sub(self, rhs: Self) -> Self::Output {
        Self (self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for UPos {
    type Output = UPos;

    fn add(self, rhs: Self) -> Self::Output {
        Self (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<usize> for UPos {
    type Output = UPos;

    fn mul(self, rhs: usize) -> Self::Output {
        Self (self.0 * rhs, self.1 * rhs)
    }
}

pub struct UPosIter {
    pos: UPos,
    width: usize,
}

impl UPosIter {
    pub fn new(width: usize) -> Self {
        Self {
            pos: UPos::new(0, 0),
            width
        }
    }
}

impl Iterator for UPosIter {
    type Item = UPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.1 < self.width {
            let ret = Some(self.pos);

            self.pos.1 += 1;
            ret
        } else {
            self.pos.1 = 0;
            self.pos.0 += 1;

            let ret = Some(self.pos);

            self.pos.1 += 1;
            ret
        }
    }
}

