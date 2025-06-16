pub fn capitalize_first(input: &str) -> String {
    if input.is_empty(){
        return "".to_string();
    }
    input.chars().next().unwrap().to_uppercase().to_string()+ &input[1..]

}

pub fn title_case(input: &str) -> String {
    let mut new = vec![];
    for word in input.split_whitespace(){
        new.push(capitalize_first(word));
    }
    new.join(" ")
}

pub fn change_case(input: &str) -> String {
    let mut new = String::new();
    for c in input.chars(){
        if c.is_uppercase(){
            new.push(c.to_lowercase().next().unwrap());
        }else if !c.is_numeric(){
            new.push(c.to_uppercase().next().unwrap());
        }
    }
    new
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!("{}", capitalize_first("joe is missing"));
        println!("{}", title_case("jill is leaving A"));
            println!("{}", change_case("heLLo THere"));
    }
}
