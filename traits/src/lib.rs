use std::fmt;
use std::fmt::Formatter;
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub strength: f64,
    pub score: i32,
    pub money: i32,
    pub weapons: Vec<String>,
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,
}

impl Player {
    pub fn eat<T: Food>(&mut self, food: T) {
        self.strength += food.gives();
    }
}

pub trait Food {
    fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        let res = self.weight_in_kg * 4 as f64;
        return res;
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let prot = ((1 as f64 - self.fat_content)* self.weight_in_kg)*4 as f64;
        let fat = (self.weight_in_kg *self.fat_content)*9 as f64;
        return prot + fat;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let res = format!(
            "{}\nStrength: {}, Score: {}, Money: {}\nWeapons: {:?}",
            self.name, self.strength, self.score, self.money, self.weapons
        );
        write!(f, "{}", res)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let apple = Fruit { weight_in_kg: 1.0 };

        println!("this apple gives {} units of strength", apple.gives());

        let steak = Meat {
            weight_in_kg: 1.0,
            fat_content: 1.0,
        };

        let mut player1 = Player {
            name: String::from("player1"),
            strength: 1.0,
            score: 0,
            money: 0,
            weapons: vec![String::from("knife")],
        };
        println!("Before eating {:?}", player1);
        player1.eat(apple);
        println!("After eating an apple\n{}", player1);
        player1.eat(steak);
        println!("After eating a steak\n{}", player1);
    }
}
