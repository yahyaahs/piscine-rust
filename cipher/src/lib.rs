#[derive(Debug, PartialEq)]
pub struct CipherError {
    // expected public fields
    pub expected : String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut s = String::new();
    for c in original.bytes(){
        if c >= 65 && c <=90{
            s.push(('Z' as u8 - c +'A' as u8)as char);
        }else if c >=97 && c <=122{
            s.push(('z' as u8 - c +'a' as u8)as char);
        }else{
            s.push(c as char)
        }
    }
    if s == ciphered.to_string(){
        return Ok(());
    }
    return Err(CipherError{ expected : s});
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    println!("{:?}", cipher("1Hello 2world!", "svool"));
    }
}
