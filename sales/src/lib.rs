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
    pub buy: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            buy: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let find_price = |s: &Store| s.products.iter().find(|x| x.0 == ele).map(|p| p.1).unwrap();
        self.buy.push((ele.clone(), find_price(s)));
        self.receipt.push(find_price(s));
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        &self.buy.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let count = self.buy.len() / 3;
        let find_cheapest = |v: &Vec<(String, f32)>| v.iter().take(count).map(|x| x.1).sum::<f32>();
        let base = find_cheapest(&self.buy);
        let total: f32 = self.buy.iter().map(|x| x.1).sum();
        let reduction = |price: f32| {
            let discounted = price - (price / total * base);
            (discounted * 100.0).round() / 100.0
        };
        let rec: Vec<f32> = self.receipt.iter().map(|&el| reduction(el)).collect();
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
