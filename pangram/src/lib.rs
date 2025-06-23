use std::collections::HashSet;
pub fn is_pangram(s: &str) -> bool {

    let mut hash = HashSet::new();

    let _: Vec<_> = s.chars().map(|c|{
        if c.is_ascii_alphabetic(){
            hash.insert(c.to_ascii_lowercase());
        }
    }).collect();
    return hash.len()==26;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
            println!(
        "{}",
        is_pangram("the quick brown fox jumps over the lazy dog!")
    );
    println!("{}", is_pangram("this is not a pangram!"));
    }
}