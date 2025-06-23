pub fn pig_latin(text: &str) -> String {
    if text.starts_with(['a','e','i','o','u','A','E','I','O','U']){
        return format!("{}ay", text);
    }
    if !text.starts_with(['a','e','i','o','u','A','E','I','O','U']) && text[1..3].contains("qu"){
        return format!("{}{}ay", text[3..].to_string(), text[0..3].to_string());
    }
    let  s= String::from(text);
    let mut v = String::new();
    for (i, c) in text.char_indices(){
        if "aeiouAEIOU".contains(c){
            return  format!("{}{}ay", s[i..].to_string(), v);
        }
        v.push(c);
    }
 
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!("{}", pig_latin(&String::from("igloo")));
     println!("{}", pig_latin(&String::from("hello")));
         println!("{}", pig_latin(&String::from("square")));
    }
}