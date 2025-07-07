#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl Numbers {
    pub fn new(numbers: &[u32]) -> Self {
        Self{
            numbers,
        }
    }

    pub fn list(&self) -> &[u32] {}

    pub fn latest(&self) -> Option<u32> {}

    pub fn highest(&self) -> Option<u32> {}

    pub fn highest_three(&self) -> Vec<u32> {}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let expected = [30, 500, 20, 70];
    let n = Numbers::new(&expected);
    println!("{:?}", n.list());
    println!("{:?}", n.highest());
    println!("{:?}", n.latest());
    println!("{:?}", n.highest_three());
    }
}
