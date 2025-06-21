use std::collections::HashMap;
use std::collections::HashSet;
pub mod boss;
pub mod member;

pub use boss::*;
pub use member::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(
            name.to_string(),
            Member {
                role: Role::Associate,
                age: age,
            },
        );
    }
    pub fn steal(&mut self, targe: &mut Mob, amount: u64) {
        if targe.wealth >= amount {
            targe.wealth -= amount;
            self.wealth += amount;
        } else {
            self.wealth += targe.wealth;
            targe.wealth = 0;

        }
    }
    pub fn conquer_city(&mut self, mobs: &[&Mob], city: String) {
        let city_taken = mobs.iter().any(|mob| mob.cities.contains(&city));
        
        if !city_taken && !self.cities.contains(&city) {
            self.cities.insert(city);
        }
    }
    pub fn combat_score(&self) -> u32 {
        let score = self.members.iter().map(|(_, mem)| match mem.role {
             Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
        });
        score.sum()
    }
    pub fn attack(&mut self, target: &mut Mob) {
        let self_score = self.combat_score();
        let target_score = target.combat_score();

        let (loser, winner) = if self_score < target_score {
            (self, target)
        } else if self_score > target_score {
            (target, self)
        } else {
            (self, target)
        };

        let loser_mem_key = loser.members.iter().map(|(name, mem)| (name, mem.age)).min_by_key(|&(_, age)| age).map(|(name, _)| name.clone()).unwrap();
        loser.members.remove(&loser_mem_key);
        if loser.members.is_empty() {
            let cities_to_transfer: Vec<String> = loser.cities.iter().cloned().collect();
            for city in cities_to_transfer {
                winner.cities.insert(city.clone());
                loser.cities.remove(&city);
            }
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }
}
