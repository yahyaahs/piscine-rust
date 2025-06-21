#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role{
    Underboss,
    Soldier,
    Caporegime,
    Associate,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member{
    pub role : Role,
    pub age : u32,
}


impl Member {
    pub fn get_promotion(&mut self){
        self.role = match self.role {
            Role::Underboss => panic!(),
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Associate => Role::Soldier,
        };
    }
}