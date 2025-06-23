pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut counter = 0;
    for _ in array.iter(){
        if array[counter as usize]== key{
            return Some(counter);
        }
        counter+=1;
    }
    return None;

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let ar = [1, 3, 4, 6, 8, 9, 11];
    let f = search(&ar, 6);
    println!(
        "the element 6 is in the position {:?} in the array {:?}",
        f, ar
    );
    }
}