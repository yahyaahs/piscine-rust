pub fn add_curry(x : i32) -> impl Fn(i32)-> i32 {
    move | y | x+y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let add10 = add_curry(-10);
    let add20 = add_curry(2066);
    let add30 = add_curry(300000);

    println!("{}", add10(5));
    println!("{}", add20(195));
    println!("{}", add30(5696));
    }
}
