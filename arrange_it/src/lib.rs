pub fn arrange_phrase(phrase: &str) -> String {
    let mut i = 1;
    let mut new= vec![];
    while new.len() < phrase.split_whitespace().count(){
        for words in phrase.split_whitespace(){
            if words.contains(&i.to_string()){
                new.push(words.replace(&i.to_string(), ""));
                i+=1;
            }
        }

    }
    new.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
                println!("the phrase {}", arrange_phrase("is2 Thi1s T4est 3a"));
    }
}
