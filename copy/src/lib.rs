pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut new_str = String::new();
    for c in a.split_whitespace(){
        new_str+= &(((c.parse::<f64>().unwrap()).exp()).to_string()+" ");
    }
    (a, (new_str.trim()).to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut new_vec= vec![];
    for nums in b.iter(){
        new_vec.push((*nums as f64).abs().ln());
    }
    return (b, new_vec);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let a = "1 2 4 5 6".to_owned();
    let b = vec![1, 2, 4, 5];
    let c = 0;

    println!("{:?}", nbr_function(c));
    println!("{:?}", vec_function(b));
    println!("{:?}", str_function(a));
    }
}
