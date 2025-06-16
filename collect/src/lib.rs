pub fn bubble_sort(arr: &mut [i32]) {
    while !arr.is_sorted(){
        for i in 1..arr.len(){
            if arr[i]< arr[i-1]{
                arr.swap(i, i-1)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let mut v = [3, 2, 4, 5, 1, 7];
    let mut v_clone = v;

    bubble_sort(&mut v);
    println!("{:?}", v);

    v_clone.sort_unstable();
    println!("{:?}", v_clone);
    }
}
