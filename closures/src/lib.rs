
pub fn first_fifty_even_square() -> Vec<i32> {

    let is_even = |n : i32| n % 2 == 0;
    let mut  v = Vec::new();
    let mut count = 1;
    while v.len() <50{
        if is_even(count){
            v.push(count *count);
        }
        count+=1;
    }
    v
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let v1 = first_fifty_even_square();

	println!("All elements in {:?}, len = {}", v1, v1.len());
    }
}
