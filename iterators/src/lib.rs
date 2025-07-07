#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;

fn next(&mut self) -> Option<Self::Item> {
    if self.v == 0 {
        return None;
    }

    let current = self.v;

    if self.v == 1 {
        self.v = 0; 
    } else if self.v % 2 == 0 {
        self.v /= 2;
    } else {
        self.v = 3 * self.v + 1;
    }

    return Some(current);
}

}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut collatz_iter = Collatz::new(n);
    let mut count = 0;

    while let Some(x) = collatz_iter.next() {
        if x == 1 {
            break;
        }
        count += 1;
    }

    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
            println!("{:?}", collatz(0));
    println!("{:?}", collatz(1));
    println!("{:?}", collatz(4));
    println!("{:?}", collatz(5));
    println!("{:?}", collatz(6));
    println!("{:?}", collatz(7));
    println!("{:?}", collatz(12));
    }
}
