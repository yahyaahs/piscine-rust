 use std::io;
fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();
    let output = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let mut i = 1;
    loop {
        println!("{}", output);
        input.clear();
        let _ = stdin.read_line(input);
        if input == "The letter e\n" {
            println!("Number of trials: {}", i);
            break;
        }
         i+=1;
    }
}
