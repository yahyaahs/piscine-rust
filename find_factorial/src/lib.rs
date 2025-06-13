pub fn factorial(num: u64) -> u64 {
    if num < 2{
        return 1;
    }else {
        num*factorial(num-1)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
