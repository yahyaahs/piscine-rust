pub fn number_logic(num: u32) -> bool {
    let mut sum = 0;
    for c in num.to_string().chars(){
        if !c.is_digit(10){
            return false;
        }
        sum+= c.to_string().parse::<u32>().unwrap().pow(num.to_string().len() as u32);
    }
    return sum == num;
}  

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
    let array = [9, 10, 153, 154];
    for pat in &array {
        if number_logic(*pat) == true {
            println!(
                "this number returns {} because the number {} obey the rules of the sequence",
                number_logic(*pat),
                pat
            )
        }
        if number_logic(*pat) == false {
            println!("this number returns {} because the number {} does not obey the rules of the sequence", number_logic(*pat),pat )
        }
    }    }
}
