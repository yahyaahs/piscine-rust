use std::ops::Add;
use std::cmp::PartialOrd;
pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
}

impl <T>StepIterator<T> {
	pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
        }
	}
}

impl <T : Add<Output = T>+PartialOrd + Clone>std::iter::Iterator for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.current > self.end {
            None
        } else {
            let current = self.current.clone();
            self.current = self.current.clone() + self.step.clone();
            Some(current)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	for v in StepIterator::new(0, 100, 10) {
		print!("{},", v);
	}
	println!();

	for v in StepIterator::new(0, 100, 12) {
		print!("{},", v)
	}
	println!();
    }
}
