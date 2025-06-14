pub fn sum(a: &[i32]) -> i32 {
    let mut sum = 0;
    for n in a {
        sum+= n;
    }
    sum
}

pub fn thirtytwo_tens() -> [i32; 32] {
    let mut arr = [0; 32];
    for i in 0..32 {
        arr[i]= 10
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
