pub fn scytale_cipher(message: String, i: u32) -> String {
    let mut msg = message.to_string();
    let mut inc = String::new();

    while msg.len()%i as usize != 0{
        msg.push('*');
    }

    let  rows = msg.len()/i as usize;

    for c in 0..i{
        for r in 0..rows{
            inc.push(msg.chars().nth(r*i as usize + c as usize).unwrap());


        }
    } 
    return inc.replace("*", " ").trim().to_string();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher(String::from("scytale Code"), 6));
    println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher(String::from("scytale Code"), 8));
}
}