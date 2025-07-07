pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut sum = 1;
    let mut count = 0;
    let mut v = Vec::new();
    if arr.len()==1{
        return arr;
    }
    while arr.len() > v.len() {
        for (i, el) in arr.iter().enumerate() {
            if count == i {
                continue;
            }
            sum *= el;
        }
        v.push(sum);
        sum = 1;
                count += 1
    }

    v
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr: Vec<usize> = vec![1, 7, 3, 4];
        let output = get_products(arr);
        println!("{:?}", output);
    }
}
