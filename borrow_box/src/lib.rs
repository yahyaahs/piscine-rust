
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession{
            id,
            p1: (p1_name , 0),
            p2:(p2_name, 0) ,
            nb_games,
        })
    }
    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 > self.p2.1{
            return self.p1.clone();
        }else if self.p1.1< self.p2.1 {
            return self.p2.clone();
        } else{
            return ("Same score! tied".to_string(), self.p1.1);
        }


    }
    pub fn update_score(&mut self, user_name: String) {
        if self.p1.1 + self.p2.1 >= self.nb_games/2+1 {
            return
        }
        if self.p1.0 == user_name{
            self.p1.1 += 1;
        }else if self.p2.0 == user_name {
            self.p2.1+=1;
        } 

    }
    pub fn delete(self) -> String {
        return format!("game deleted: id -> {}", self.id);

    }
}
#[cfg(test)]
mod tests {
    use super::*;
      fn create_games() -> Vec<Box<GameSession>> {
        vec![
            GameSession::new(0, String::from("player1"), String::from("player2"), 1),
            GameSession::new(1, String::from("Alice"), String::from("Mark"), 3),
            GameSession::new(2, String::from("Jack"), String::from("Miller"), 5),
        ]
    }


    #[test]
    fn it_works() {
            let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
    println!("{:?}", game.read_winner());
    // output : ("Same score! tied", 0)

    game.update_score(String::from("Susana"));
    game.update_score(String::from("Susana"));
    game.update_score(String::from("Susaa"));

    // println!("{:?}", game.read_winner());
    // output : ("Same score! tied", 2)

    // this one will not count because it already 5 games played, the nb_games
    game.update_score(String::from("Susna"));
    game.update_score(String::from("Susa"));
    game.update_score(String::from("Susan"));



    println!("{:?}", game.read_winner());
    // output : ("Joao", 3)

    println!("{:?}", game.delete());
    // output : "game deleted: id -> 0"

    // game.read_winner();
    // this will give error
    // because the game was dropped, no longer exists on the heap
    }

    #[test]
    fn test_stop_updating() {
        let mut games = create_games();
        games[0].update_score(String::from("player1"));
        games[0].update_score(String::from("player1"));
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));

        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        assert_eq!(games[2].read_winner(), (String::from("Jack"), 3));
    }
}
