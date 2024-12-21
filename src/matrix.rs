use std::{collections::{HashSet, VecDeque}, fmt::{Debug, Display}, iter::Zip, ops::{Index, IndexMut, Mul}};
use crate::position::{Pos, UPos, PosIter, UPosIter, Dir};

#[derive(Debug, Clone, Copy)]
struct AStarCell {
    parent: Option<UPos>,
    f: usize,
    g: usize,
    h: usize,
}

impl AStarCell {
    fn new() -> Self {
        Self { parent: None, f: usize::MAX, g: usize::MAX, h: usize::MAX }
    }
}

type AStarPair = (usize, UPos);

pub struct Matrix<T> {
    rows: Vec<T>,
    row_count: usize,
    width: usize,
}

type Cost = u32;
impl<T> Matrix<T> {
    pub fn new(rows: Vec<T>, width: usize) -> Self {
        assert!(!rows.is_empty());
        assert!(rows.len() % width == 0);

        let row_count = rows.len() / width;
        Self { rows, row_count, width }
    } 

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get(&self, pos: &UPos) -> Option<&T> {
        let UPos(x,y) = *pos;

        if (0..self.width).contains(&y) && (0..self.row_count).contains(&x) {
            self.rows.get(x * self.width + y)
        } else {
            None
        }    
    }

    pub fn get_pos(&self, pos: &Pos) -> Option<&T> {
        let x = pos.0 as usize;
        let y = pos.1 as usize;

        if (0..self.width).contains(&y) && (0..self.row_count).contains(&x) {
            self.rows.get(x * self.width + y)
        } else {
            None
        }    
    }

    pub fn get_mut(&mut self, pos: &UPos) -> Option<&mut T> {
        let UPos(x,y) = pos;

        if (0..self.width).contains(y) && (0..self.row_count).contains(x) {
            self.rows.get_mut(x * self.width + y)
        } else {
            None
        }
    }

    pub fn get_mut_pos(&mut self, pos: &Pos) -> Option<&mut T> {
        let x = pos.0 as usize;
        let y = pos.1 as usize;

        if (0..self.width).contains(&y) && (0..self.row_count).contains(&x) {
            self.rows.get_mut(x * self.width + y)
        } else {
            None
        }
    }
    
    pub fn look_ahead(&self, upos: &UPos, dir: &Dir) -> Option<(&T, UPos)> {
        let offset = Pos::from(*upos) + Pos::from(*dir);
        let val = self.get_pos(&offset)?;

        // We know it's a valid UPos because it's inside the matrix
        let offset = offset.try_into().unwrap();
        Some((val, offset))
    }

    pub fn look_ahead_mut(&mut self, upos: &UPos, dir: &Dir) -> Option<(&mut T, UPos)> {
        let offset = Pos::from(*upos) + Pos::from(*dir);
        let val = self.get_mut_pos(&offset)?;

        // We know it's a valid UPos because it's inside the matrix
        let offset = offset.try_into().unwrap();
        Some((val, offset))
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.rows.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.rows.iter_mut()
    }

    pub fn give_upos(&self) -> Zip<std::slice::Iter<'_, T>, UPosIter> {
        self.iter().zip(UPosIter::new(self.width))
    }

    pub fn give_upos_mut(&mut self) -> Zip<std::slice::IterMut<'_, T>, UPosIter> {
        let width = self.width;
        self.iter_mut().zip(UPosIter::new(width))
    }

    pub fn give_pos(&self) -> Zip<std::slice::Iter<'_, T>, PosIter> {
        self.iter().zip(PosIter::new(self.width))
    }

    pub fn give_pos_mut(&mut self) -> Zip<std::slice::IterMut<'_, T>, PosIter> {
        let width = self.width;
        self.iter_mut().zip(PosIter::new(width))
    }

    /// Incomplete right now
    pub fn a_star(&self, start: UPos, end: UPos) -> Option<(Vec<UPos>, Cost)> {
        if self.get(&start).is_none() {
            eprintln!("{start:?} is not a valid position");
            return None;
        }

        if self.get(&end).is_none() {
            eprintln!("{end:?} is not a valid position");
            return None;
        }

        let mut cost = 0;
        let mut path = Vec::new();

        if start == end {
            return Some((path, cost));
        } 

        let mut closed = Matrix::with_capacity(self.row_count, self.width, false);
        let mut cell_details = Matrix::with_capacity(self.row_count, self.width, AStarCell::new());

        // initialize 
        cell_details[start].parent = Some(start);
        cell_details[start].f = 0;
        cell_details[start].g = 0;
        cell_details[start].h = 0;

        let mut open: VecDeque<AStarPair> = VecDeque::new();
        let mut open_set: HashSet<AStarPair> = HashSet::new();

        let first: AStarPair = (0, start);
        open.push_back(first);
        open_set.insert(first);

        let mut found_dest = false;

        let card_dir = Mask::new(vec![Pos(-1,0),Pos(1,0),Pos(0,-1),Pos(0,1)]);
        while !open.is_empty() {
            let p@(_f,pos): AStarPair = open.pop_front()?;
            open_set.remove(&p);

            closed[pos] = true;

            // Now we check the 4 cardinal directions:
            // .  N   .
            // W cell E
            // .  S   .

            let (g_new, h_new, f_new): (usize, usize, usize);

            let dirs = card_dir.apply_upos(pos, &cell_details);

            for dir in dirs {
                if let Some(cell) = dir {
                    
                }
            }
        }

        Some((path, cost))
    }
}

impl<T> Index<Pos> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        self.get_pos(&index).unwrap()
    }
}

impl<T> IndexMut<Pos> for Matrix<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        self.get_mut_pos(&index).unwrap()
    }
}

impl<T> Index<UPos> for Matrix<T> {
    type Output = T;

    fn index(&self, index: UPos) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<T> IndexMut<UPos> for Matrix<T> {
    fn index_mut(&mut self, index: UPos) -> &mut Self::Output {
        self.get_mut(&index).unwrap()
    }
}

impl<T: Clone> Matrix<T> {
    /// Create a Matrix with given size, filled with a 'neutral' value
    pub fn with_capacity(row_count: usize, width: usize, neutral: T) -> Self {
        let mut rows = Vec::with_capacity(row_count*width);
        for _i in 0..rows.capacity() {
            rows.push(neutral.clone());
        }
        Self {
            rows,
            row_count,
            width,
        }
    }
}

impl<T: Into<f64>> Mul<Matrix<T>> for f64 {
    type Output = Matrix<f64>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let mut rows = Vec::new();
        for val in rhs.rows {
            rows.push(val.into() * self);
        }
        Matrix::new(rows, rhs.width)
    }
}   

impl Matrix<f64> {
    pub fn augment(&self, rhs: Self) -> Self {
        let mut new_row = Vec::new();
        for row in 0..self.row_count {
            for col in 0..self.width {
                new_row.push(self[UPos(row,col)]);
            }
            for col in 0..rhs.width() {
                new_row.push(rhs[UPos(row,col)]);
            }
        }

        Self { rows: new_row, row_count: self.row_count, width: self.width + rhs.width() }
    }

    pub fn split_at(&self, col: usize) -> Self {
        let mut new_row = Vec::new();

        for row in 0..self.row_count {
            for og_col in 0..self.width {
                if og_col >= col {
                    new_row.push(self[UPos(row,og_col)]);
                }
            }
        }

        Self { rows: new_row, row_count: self.row_count, width: self.width - col }
    }

    pub fn gauss_jordan_inverse(&self) -> Option<Self> {
        if self.row_count != self.width {
            return None;
        }

        let mut inverse = self.clone().augment(Matrix::identity(1.0, self.width));
        println!("inverse_width={}",inverse.width);

        let mut h = 0; // pivot row
        let mut k = 0; // pivot col

        // get to echelon form
        while h < inverse.row_count && k < inverse.width {
            let row_max = argmax(&inverse, k).unwrap() ;

            if is_near_zero(&inverse[UPos(row_max,k)]) {
                // no pivot, pass to next column
                k += 1;
            } else {
                inverse.swap_rows(h,row_max);
                inverse.mul_row(h, 1./inverse[UPos(h,k)]);

                // do for all rows below pivot:
                for i in h+1..inverse.row_count {
                    let f = -(inverse[UPos(i,k)] / inverse[UPos(h,k)]);
                    // fill with zeros the lower part of pivot column
                    inverse[UPos(i,k)] = 0.0;
                    // do for all remaining elements in current row
                    for j in k+1..inverse.width {
                        inverse[UPos(i,j)] = inverse[UPos(i,j)] + inverse[UPos(h,j)] * f;
                    }
                }

                h += 1;
                k += 1;
            }
        }

        // do the Jordan part. Start from the end
        let mut h = self.row_count-1;
        let mut k = self.width-1;

        while h > 0 && k > 0 {
            for row in 0..h {
                let f = -(inverse[UPos(row,k)] / inverse[UPos(h,k)]);
                inverse.add_rows(row, h, f);
            }

            h -= 1;
            k -= 1;
        }

        println!("whole_matrix =\n{:#?}", inverse);
        
        let inverse = inverse.split_at(self.width);
        Some(inverse)
    }

    pub fn identity(val: f64, dimension: usize) -> Matrix<f64> {
        let mut rows = Vec::new();
        let width = dimension;
        
        for row in 0..width {
            for col in 0..width {
                if row==col {
                    rows.push(val);
                } else {
                    rows.push(0.0);
                }
            }
        }

        Matrix::new(rows, width)
    }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        for i in 0..self.width()  {
            let aux = self[UPos(a,i)];
            self[UPos(a,i)] = self[UPos(b,i)];
            self[UPos(b,i)] = aux;
        }
    }

    pub fn add_rows(&mut self, a: usize, b: usize, k: f64) {
        for i in 0..self.width()  {
            let val = self[UPos(b,i)] * k;
            self[UPos(a,i)] += val;
        }
    }

    pub fn mul_row(&mut self, a: usize, val: f64) {
        for i in 0..self.width()  {
            self[UPos(a,i)] *= val;
        }
    }
}

pub fn manhattan_distance(start: &UPos, end: &UPos) -> usize {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}

fn is_near_zero(val: &f64) -> bool {
    *val <= 1e-3
}

fn argmax<T: std::cmp::PartialOrd + Clone>(matrix: &Matrix<T>, col: usize) -> Option<usize> {
    if matrix.row_count() > 0 && col < matrix.width() {
        let column = col ;
        let mut max_row: usize = 0;
        let mut max_val = matrix[UPos(max_row,column)].clone();

        for row in 1..matrix.row_count()  {
            let val = matrix[UPos(row,column)].clone();
            if max_val.partial_cmp(&val).unwrap() == std::cmp::Ordering::Less {
                max_row = row;
                max_val = val;
            }
        }

        Some(max_row)
    } else {
        None
    }
}


impl Mul for Matrix<i64> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.width != rhs.row_count {
            None
        } else {
            let mut rows = Vec::new();
            for row in 0..self.row_count  {
                for col in 0..rhs.width()  {
                    let mut value = 0;
                    for pos in 0..self.width  {
                        value += self[UPos(row,pos)] * self[UPos(pos,col)];
                    }

                    rows.push(value);
                }
            }

            Some(Matrix::new(rows, rhs.width()))
        }
    }
}

impl Mul for Matrix<f64> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.width != rhs.row_count {
            None
        } else {
            let mut rows = Vec::new();
            for row in 0..self.row_count  {
                for col in 0..rhs.width()  {
                    let mut value = 0.0;
                    for pos in 0..self.width  {
                        value += self[UPos(row,pos)] * self[UPos(pos,col)];
                    }

                    rows.push(value);
                }
            }

            Some(Matrix::new(rows, rhs.width()))
        }
    }
}

impl <T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Self { rows: self.rows.clone(), row_count: self.row_count, width: self.width }
    }
}

impl<T: Clone + Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.row_count {
            let start = i*self.width;
            let row: Vec<T> = self.rows[start..start+self.width].to_vec();
            for item in row {
                write!(f, "{item}")?;
            }
            writeln!(f)?
        }

        Ok(())
    }
}

impl<T: Debug + Clone> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.row_count {
            let start = i*self.width;
            let row: Vec<T> = self.rows[start..start+self.width].to_vec();
            for item in row {
                write!(f, "{item:?}")?;
            }
            writeln!(f)?
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
                matrix.get_pos(&rel_pos)
            })
            .collect()
    }

    pub fn apply_upos<'a, T>(&self, upos: UPos, matrix: &'a Matrix<T>) -> Vec<Option<&'a T>> {
        self.relative_pos
            .iter()
            .map(|&i| {
                let rel_pos = i+Pos::from(upos);
                matrix.get_pos(&rel_pos)
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
            let neighbours:Vec<Option<char>> = mask.apply(pos, &matrix)
                .iter()
                .map(|v| v.as_deref().copied())
                .collect();
            if pos == Pos(1,1) {
                assert_eq!(
                    neighbours, 
                    vec![Some('R'),Some('R'),Some('R'),Some('R')]
                );
            }

            if pos == Pos(1,0) {
                assert_eq!(
                    neighbours, 
                    vec![Some('A'),Some('A'),None,Some('R')]
                );
            }

            println!("neighbours of {val} = {neighbours:?}");
        }
    }

    #[test]
    fn test_gauss_jordan_inverse() {
        let rows = vec![2.,-1.,0.,-1.,2.,-1.,0.,-1.,2.];
        let width = 3;

        let matrix = Matrix::new(rows, width);

        let inverse = matrix.gauss_jordan_inverse().unwrap();
        println!("inverse = \n{inverse:#?}");

        let mut diff = 0.0;
        for (val, upos) in matrix.give_upos() {
            diff += *val - inverse[upos];
        }
        assert!(is_near_zero(&diff));
    }
}
