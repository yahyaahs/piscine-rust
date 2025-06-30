#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
	pub i: T,
	pub j: T,
	pub k: T,
}

use std::ops::{Add, Sub};

impl<T: Add<Output = T>> Add for ThreeDVector<T> {
    type Output = ThreeDVector<T>;

    fn add(self, other: ThreeDVector<T>) -> ThreeDVector<T> {
        ThreeDVector {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

impl<T: Sub<Output = T>> Sub for ThreeDVector<T> {
    type Output = ThreeDVector<T>;

    fn sub(self, other: ThreeDVector<T>) -> ThreeDVector<T> {
        ThreeDVector {
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let a = ThreeDVector { i: 3, j: 5, k: 2 };
	let b = ThreeDVector { i: 2, j: 7, k: 4 };
	println!("{:?}", a + b);
    }
}
