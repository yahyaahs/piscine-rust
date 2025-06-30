use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("check"),
        }
    }
}

impl From<u32> for RomanNumber {

    fn from(mut value: u32) -> Self {
        let mut digits = Vec::new();
        if value == 0 {
            return RomanNumber(vec![Nulla]);
        }
        while value > 0 {
            if value >= 1000 {
                digits.push(M);
                value -= 1000;
            } else if value >= 900 {
                digits.push(C);
                digits.push(M);
                value -= 900;
            } else if value >= 500 {
                digits.push(D);
                value -= 500;
            } else if value >= 400 {
                digits.push(C);
                digits.push(D);
                value -= 400;
            } else if value >= 100 {
                digits.push(C);
                value -= 100;
            } else if value >= 90 {
                digits.push(X);
                digits.push(C);
                value -= 90;
            } else if value >= 50 {
                digits.push(L);
                value -= 50;
            } else if value >= 40 {
                digits.push(X);
                digits.push(L);
                value -= 40;
            } else if value >= 10 {
                digits.push(X);
                value -= 10;
            } else if value >= 9 {
                digits.push(I);
                digits.push(X);
                value -= 9;
            } else if value >= 5 {
                digits.push(V);
                value -= 5;
            } else if value >= 4 {
                digits.push(I);
                digits.push(V);
                value -= 4;
            } else if value >= 1 {
                digits.push(I);
                value -= 1;
            }
        }
        RomanNumber(digits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	println!("{:?}", RomanNumber::from(32));
	println!("{:?}", RomanNumber::from(9));
	println!("{:?}", RomanNumber::from(45));
	println!("{:?}", RomanNumber::from(0));
    }
}
