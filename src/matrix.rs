use std::{fmt::Debug, iter::Zip};
use crate::position::{Pos, PosIter};

#[derive(Clone)]
pub struct Matrix<T> {
    rows: Vec<T>,
    row_count: usize,
    width: usize,
}

impl<T> Matrix<T> {
    pub fn new(rows: Vec<T>, width: usize) -> Self {
        assert!(!rows.is_empty());
        assert!(rows.len() % width == 0);

        let row_count = rows.len() / width;
        Self { rows, row_count, width }
    }

    pub fn with_capacity(row_count: usize, width: usize) -> Self {
        Self {
            rows: Vec::with_capacity(row_count*width),
            row_count,
            width,
        }
    }

    pub fn push(&mut self, value: T) {
        assert!(self.rows.len() <= self.row_count*self.width);

        self.rows.push(value);
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get(&self, pos: &Pos) -> Option<&T> {
        let x = pos.0 as usize;
        let y = pos.1 as usize;

        if (0..self.width).contains(&y) && (0..self.row_count).contains(&x) {
            self.rows.get(x * self.width + y)
        } else {
            None
        }    
    }

    pub fn get_mut(&mut self, pos: &Pos) -> Option<&mut T> {
        let x = pos.0 as usize;
        let y = pos.1 as usize;

        if (0..self.width).contains(&y) && (0..self.row_count).contains(&x) {
            self.rows.get_mut(x * self.width + y)
        } else {
            None
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.rows.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.rows.iter_mut()
    }

    pub fn give_pos(&self) -> Zip<std::slice::Iter<'_, T>, PosIter> {
        self.iter().zip(PosIter::new(self.width))
    }

    pub fn give_pos_mut(&mut self) -> Zip<std::slice::IterMut<'_, T>, PosIter> {
        let width = self.width;
        self.iter_mut().zip(PosIter::new(width))
    }
}

impl<T: Debug + Clone> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.row_count {
            let start = i*self.width;
            let row: Vec<T> = self.rows[start..start+self.width].to_vec();
            writeln!(f, "{:?}", row)?
        }

        Ok(())
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter_mut()
    }
}

/*
* instead of implementing the iterator directly, we should
* create a struct for iterating the Matrix and implementing
* into_iter for Matrix into that struct.
impl Iterator for Matrix {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.1+1 < self.width as i32 {
            let pos = self.pos;
            self.pos.1 += 1;

            self.get(&pos).copied()
        } else if self.pos.0 < self.row_count as i32 {
            let pos = self.pos;
            self.pos.0 += 1;
            self.pos.1 = 0;

            self.get(&pos).copied()
        } else {
            None
        }
    }
}
*/

pub struct Mask {
    relative_pos: Vec<Pos>,
}

impl Mask {
    pub fn new(relative_pos: Vec<Pos>) -> Self {
        Self {
            relative_pos,
        }
    }

    pub fn apply<'a, T>(&self, pos: Pos, matrix: &'a Matrix<T>) -> Vec<Option<&'a T>> {
        self.relative_pos
            .iter()
            .map(|&i| {
                let rel_pos = i+pos;
                matrix.get(&rel_pos)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_apply() {
        let input = "\
ARA
RRR
ARA";
        let width = 3;
        let rows = input.replace("\n", "").chars().collect();

        let matrix = Matrix::new(rows, width);

        let mask = Mask::new(vec![Pos(-1,0),Pos(1,0),Pos(0,-1),Pos(0,1),]);
        for (val, pos) in matrix.give_pos() {
            let neighbours = mask.apply(pos, &matrix);
            if pos == Pos(1,1) {
                assert_eq!(neighbours, vec![matrix.get(&Pos(0,1)),matrix.get(&Pos(2,1)),matrix.get(&Pos(1,0)),matrix.get(&Pos(1,2)),]);
            }

            println!("neighbours of {val} = {neighbours:?}");
        }
    }
}
