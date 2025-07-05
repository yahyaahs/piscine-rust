pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    match s.strip_prefix(prefix){
        Some(item)=> return Some(item),
        None=> None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	println!("{:?}", delete_prefix("ab", "abcdefghijklmnop"));
	println!("{:?}", delete_prefix("x", "abcdefghijklmnop"));
    }
}
