
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
use std::{ops::{Add, Div, Mul, Sub}, process::Output};
pub trait Scalar: Copy+ Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> {
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}

impl Scalar for u32{
    type Item = u32;
    fn one() -> Self::Item {
        1
    }
    fn zero() -> Self::Item {
        0
    }
}
impl Scalar  for u64{
    type Item = u64;
    fn one() -> Self::Item {
        1
    }
    fn zero() -> Self::Item {
        0
    }
}
impl Scalar for i32 {
    type Item = i32;
    fn one() -> Self::Item {
        1
    }
    fn zero() -> Self::Item {
        0
    }
    
}
impl Scalar for i64 {
    type Item = i64;
    fn one() -> Self::Item {
        1
    }
    fn zero() -> Self::Item {
        0
    }
}
impl Scalar for f32 {
    type Item = f32;
    fn one() -> Self::Item {
        1.0
    }
    fn zero() -> Self::Item {
        0.0
    }
}
impl Scalar for f64 {
    type Item = f64;
    fn one() -> Self::Item {
        1.0
    }
    fn zero() -> Self::Item {
        0.0
    }
}

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col];row])
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut m = Matrix::zero(n, n);
        for i in 0..n{
            m.0[i][i] = T::one();
        }
        return m;
	}
}


impl <T : Scalar<Item = T >>Sub for Matrix<T> {
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::zero(self.0.len(), self.0.len());
        if self.0.len() != rhs.0.len() || self.0[0].len()!= rhs.0[0].len(){
            return None;
        }else {
            for i in 0..self.0.len(){
                for j in 0..self.0.len(){
                    res.0[i][j] = self.0[i][j]- rhs.0[i][j];
                }
            }
        }
        return Some(res);
    }
}


impl <T : Scalar<Item = T >>Add for Matrix<T> {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::zero(self.0.len(), self.0.len());
        if self.0.len() != rhs.0.len()|| self.0[0].len()!= rhs.0[0].len(){
            return None;
        }else {
            for i in 0..self.0.len(){
                for j in 0..self.0.len(){
                    res.0[i][j] = self.0[i][j]+ rhs.0[i][j];
                }
            }
        }
        return Some(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let matrix = Matrix(vec![vec![8, 1], vec![9, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
	println!("{:?}", matrix + matrix_2);

	let matrix = Matrix(vec![vec![1, 3], vec![2, 5]]);
	let matrix_2 = Matrix(vec![vec![3, 1], vec![1, 1]]);
	println!("{:?}", matrix - matrix_2);

	let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
	println!("{:?}", matrix - matrix_2);

	let matrix = Matrix(vec![vec![1, 3], vec![9, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
	println!("{:?}", matrix + matrix_2);
    }
}
