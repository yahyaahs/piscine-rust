use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    *h.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let hash = HashMap::from_iter([
        ("Daniel", 122),
        ("Ashley", 333),
        ("Katie", 334),
        ("Robert", 14),
    ]);

    println!(
        "The biggest of the elements in the HashMap is {}",
        bigger(hash)
    );
    }
}
