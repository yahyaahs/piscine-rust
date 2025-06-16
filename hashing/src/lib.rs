use std::collections::HashMap;
pub fn mean(list: &[i32]) -> f64 {
    let sum :i32=  list.iter().sum();
   sum as f64 /list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();
    if sorted.len()%2==0{
        sorted[sorted.len()/2-1] + sorted[sorted.len()/2]
    }else{
        sorted[sorted.len()/2]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut hash = HashMap::new();

    for num in list{
        *hash.entry(num).or_insert(0)+=1;
    }
    let max = *hash.values().max().unwrap();
    for (&k, v) in hash.iter(){
        if *v == max{
            return *k;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let v = [5, 4, 7, 5, 2, 5, 1, 3];

    println!("mean {}", mean(&v));
    println!("median {}", median(&v));
    println!("mode {}", mode(&v));
    }
}
