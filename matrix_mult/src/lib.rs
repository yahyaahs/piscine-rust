

#[derive(Debug, PartialEq, Eq,Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
use std::{ops::{Add, Div, Mul, Sub}};
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

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        return self.0[0].len();
    }

    pub fn number_of_rows(&self) -> usize {
        return self.0.len();
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        return self.0[n].clone();
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut nw = Vec::new();
        for i in 0..self.number_of_rows() {
            for j in 0..self.number_of_cols() {
                if j == n {
                    nw.push(self.0[i][n].clone());
                }
            }
        }
        return nw;
    }
}

impl<T: Scalar<Item= T> + Mul<Output = T>  > Mul for Matrix<T> {
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        let a_col = self.number_of_cols();
        let a_row = self.number_of_rows();
        let b_row = rhs.number_of_cols();
        let b_cols = rhs.number_of_rows();

        if a_col != b_row {
            return None;
        }
        let mut new = Matrix::zero(a_col, b_row);
        for i in 0..a_row {
            for j in 0..b_cols {
                let mut sum = T::zero();
                for x in 0..a_col {
                    sum = sum + self.0[i][x].clone() * rhs.0[x][j].clone();
                }
                new.0[i][j] = sum;
            }
        }
        Some(new)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::{self, Debug, Display};

    // TODO: TestProperties to a lib
    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)]
    pub enum Kind {
        Method,   // Makes the message firstInput.MethodName(inputs[1], input[2], ..])
        Operator, // Makes the message inputs[0] OperatorName inputs[1] ex: 1 + 2
        Function, // Makes the message FunctionName(inputs[0], inputs[1], inputs[2], ..)
        Value,
    }

    pub struct Inputs<'a>(pub &'a [Input]);

    impl<'a> Display for Inputs<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for item in self.0.iter().take(self.0.len() - 1) {
                write!(f, "{:?}, ", item)?;
            }
            write!(f, "{:?}", self.0[self.0.len() - 1])
        }
    }

    pub type Input = Box<dyn Debug>;

    #[derive(Debug)]
    pub struct TestProperties {
        pub name: &'static str,
        pub kind: Kind,
    }

    impl TestProperties {
        pub fn assert_with_message<U: std::fmt::Debug + std::cmp::PartialEq>(
            &self,
            inputs: &[Input],
            actual: U,
            expected: U,
        ) {
            let message_name = match (inputs.len(), self.kind) {
                (0, Kind::Function) => format!("{}()", self.name),
                (0, Kind::Value) => format!("{}", self.name),
                (0, _) => String::new(),
                (1, Kind::Method) => format!("{:?}.{}()", inputs[0], self.name),
                (1, Kind::Function) => format!("{}({:?})", self.name, inputs[0]),
                (1, Kind::Operator) => format!("{} {:?}", self.name, inputs[0]),
                (_, Kind::Function) => format!("{}({:?})", self.name, inputs),
                (_, Kind::Operator) => {
                    format!("{:?} {} {}", inputs[0], self.name, Inputs(&inputs[1..]))
                }
                (_, Kind::Method) => {
                    format!("{:?}.{}({})", inputs[0], self.name, Inputs(&inputs[1..]))
                }
                (_, Kind::Value) => {
                    format!("{}.{}", Inputs(&inputs), self.name)
                }
            };
            assert_eq!(
                actual, expected,
                "\n\t`{}` == {:?}, expected == {:?}",
                message_name, actual, expected
            );
        }
    }

    #[test]
    fn get_row() {
        let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
        let test = TestProperties {
            name: "row",
            kind: Kind::Method,
        };
        test.assert_with_message(&[Box::new(matrix.clone())], vec![3u32, 6], matrix.row(0));
        test.assert_with_message(&[Box::new(matrix.clone())], vec![8u32, 0], matrix.row(1));
    }

    #[test]
    fn get_col() {
        let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
        let test = TestProperties {
            name: "col",
            kind: Kind::Method,
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.col(0), vec![3u32, 8]);
        test.assert_with_message(&[Box::new(matrix.clone())], vec![6u32, 0], matrix.col(1));
    }

    #[test]
    fn matrix_multiplication() {
        let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
        let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 0]]);
        let test = TestProperties {
            name: "*",
            kind: Kind::Operator,
        };
        test.assert_with_message(
            &[Box::new(matrix_1.clone()), Box::new(matrix_2.clone())],
            matrix_1 * matrix_2,
            Some(expected),
        );

        let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
        let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0, 0], vec![1, 0, 0], vec![1, 1, 1]]);
        test.assert_with_message(
            &[Box::new(matrix_1.clone()), Box::new(matrix_2.clone())],
            matrix_1 * matrix_2,
            None,
        );
    }
}
