use std::ops::Mul;
use std::ops::Add;
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
                return  self.0[0].len();

    }

    pub fn number_of_rows(&self) -> usize {
        return  self.0.len();
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        return self.0[n].clone();
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut nw = Vec::new();
        for i in 0..self.number_of_rows(){
            for j in 0..self.number_of_cols(){
                if j == n{
                    nw.push(self.0[i][n].clone());
                }
            }
        }
        return nw;
    }
}

impl<T: Clone + Mul<Output= T> + Add<Output = T> + Default> Mul for Matrix<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let a_col = self.number_of_cols();
        let a_row = self.number_of_rows();
        let b_row = rhs.number_of_cols();
        let b_cols =rhs.number_of_rows() ;

        if a_col != b_row{
            panic!("check size of matrices");
        }
        let mut new = vec![vec![T::default(); b_cols]; a_row];
        for i in 0..a_row{
            for j in 0..b_cols{
                let mut sum= T::default();
                for x in 0..a_col{
                    sum= sum + self.0[i][x].clone() * rhs.0[x][j].clone();
                }
                new[i][j]= sum;
            }
        }
        Matrix(new)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        	let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
	println!("{:?}", matrix.col(0));
	println!("{:?}", matrix.row(1));

	let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
	let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
	let mult = matrix_1.clone() * matrix_2.clone();
	println!("{:?}", mult);
	println!("{:?}", matrix_1.number_of_cols());
	println!("{:?}", matrix_2.number_of_rows());
    }
}
