use std::collections::HashMap;
use std::hash::Hash;
pub fn slices_to_map< 'a, T: Hash + Eq, U>(k: &'a [T],v: &'a [U]) -> HashMap<&'a T, &'a U> {

    let le = k.len().min(v.len());
    let mut map = HashMap::new();
    for i in 0..le {
        map.insert(&k[i], &v[i]);
    }
    map

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
	let values = [1, 3, 23, 5, 2];
	println!("{:?}", slices_to_map(&keys, &values));
    }
}
