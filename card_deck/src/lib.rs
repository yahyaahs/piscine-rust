use rand::Rng;
#[derive(Debug, PartialEq)]

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, PartialEq)]
pub enum Rank {
        Ace,
    King,
    Queen,
    Jack,
    Number(u8)
}

impl Suit {
    pub fn random() -> Suit {
        let random = rand::thread_rng().gen_range(1..5) as u8;
        return  Suit::translate(random);

    }

    pub fn translate(value: u8) -> Suit {
          match value {
            1 => return  Suit::Heart,
            2 => return Suit::Diamond,
            3 => return Suit::Spade,
            4 => return Suit::Club,
            _ => panic!("Invalid suit value"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let random = rand::thread_rng().gen_range(1..13) as u8;
        return  Rank::translate(random);
    }

    pub fn translate(value: u8) -> Rank {
                match value{
            1 => return Rank::Ace,
            11 => return Rank::Jack,
            12 => return Rank::Queen,
            13 => return Rank::King,
            n => Rank::Number(n),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.rank == Rank::Ace && card.suit == Suit::Spade{
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let your_card = Card {
        rank: Rank::random(),
        suit: Suit::random(),
    };

    println!("Your card is {:?}", &your_card);

    if winner_card(&your_card) {
        println!("You are the winner!");
    }
    }
}
