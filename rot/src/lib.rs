pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c|{
        match c {
            'a'..='z'=> {
                let calc = (((c as i8-b'a' as i8+key as i8)%26 as i8+26 as i8)%26) as u8;
                (b'a' + calc) as char
            }
            'A'..='Z'=> {
                {
                let calc = (((c as i8-b'A' as i8+key as i8)%26 as i8+26 as i8)%26) as u8;
                (b'A' + calc) as char
            }
            }
            _=>c as char,

        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
 println!("The letter \"a\" becomes: {}", rotate("a", 26));
    println!("The letter \"m\" becomes: {}", rotate("m", 0));
    println!("The letter \"m\" becomes: {}", rotate("m", 13));
    println!("The letter \"a\" becomes: {}", rotate("a", 15));
    println!("The word \"MISS\" becomes: {}", rotate("MISS", 5));
    println!(
        "The decoded message is: {}",
        rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13)
    );
    println!(
        "The decoded message is: {}",
        rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5)
    );
    println!(
        "Your cypher wil be: {}",
        rotate("Testing with numbers 1 2 3", 4)
    );
    println!("Your cypher wil be: {}", rotate("Testing", -14));
    println!("The letter \"a\" becomes: {}", rotate("a", -1));    }
}