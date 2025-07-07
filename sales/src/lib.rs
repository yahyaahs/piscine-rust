
#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    // expected public fields
    pub buy : Vec<(String, f32)>,
    pub receipt : Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart{
            buy : Vec::new(),
            receipt : Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let find_price = |s : &Store| s.products.iter().find(|x| x.0==ele).map(|p| p.1).unwrap();
        self.buy.push((ele.clone(), find_price(s)));
        self.receipt.push(find_price(s));
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {

    self.buy.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let find_cheapest = |v : &Vec<f32>| v.iter().copied().reduce(|a, b| if a <b { a}else{b}).unwrap();
        let base = find_cheapest(&self.receipt);
        let total:f32 = self.buy.iter().map(|x| x.1).sum();
        let reduction = |price : f32| price - (total/(total-base) as f32) as f32;
        let rec: Vec<f32> = self.receipt.iter().map(|&el | reduction(el)).collect();
        return rec;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let store = Store::new(vec![
            (String::from("product A"), 1.23),
            (String::from("product B"), 23.1),
            (String::from("product C"), 3.12),
        ]);

        println!("{:?}", store);

        let mut cart = Cart::new();
        cart.insert_item(&store, String::from("product A"));
        cart.insert_item(&store, String::from("product B"));
        cart.insert_item(&store, String::from("product C"));

        println!("receipt generated {:?}", cart.generate_receipt());

        println!("{:?}", cart);
    }
}
