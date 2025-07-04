#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
use std::ops::{Add, Sub, Mul, Div};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        	let m: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
            
            	println!("{:?}", m);
                	println!("{:?}", Matrix::<i32>::identity(4));
            println!("{:?}", Matrix::<f64>::zero(3, 4));



    }
}
