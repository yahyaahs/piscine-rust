pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut new = Vec::new();
    let mut initials = String::new();

    for words in names{
        for name in words.split(" "){
            initials+= &(String::from(name.chars().nth(0).unwrap())+". ");
        }
        new.push(String::from(initials.trim()));
        initials.clear();

    }
    return new;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
    }
}
