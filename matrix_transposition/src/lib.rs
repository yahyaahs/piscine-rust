#[derive(Debug, PartialEq, Eq)]

pub struct Matrix(pub (i32, i32), pub(i32, i32));
pub fn transpose(m: Matrix) -> Matrix {
    let mut new = m;
    let temp = new.0.1;
    new.0.1= new.1.0;
    new.1.0=temp;
    new
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Transpose matrix {:?}", transpose(matrix));
    }
}
