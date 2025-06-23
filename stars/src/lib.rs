pub fn stars(n: u32) -> String {
    let s = "*".to_string().repeat((2 as i32).pow(n as u32)as usize);
    return s;
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
