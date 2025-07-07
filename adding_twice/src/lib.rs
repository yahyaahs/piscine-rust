pub fn add_curry(x : i32) -> impl Fn(i32)-> i32 {
    move | y | x+y
}

pub fn twice<F: Fn(i32)->i32 >(f: F) -> impl Fn(i32)-> i32{
    move | x |  f(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let add10 = add_curry(10);
    let value = twice(add10);
    println!("The value is {}", value(7));

    let add20 = add_curry(20);
    let value = twice(add20);
    println!("The value is {}", value(7));

    let add30 = add_curry(30);
    let value = twice(add30);
    println!("The value is {}", value(7));

    let neg = add_curry(-32);
    let value = twice(neg);
    println!("The value is {}", value(7));
    }
}
