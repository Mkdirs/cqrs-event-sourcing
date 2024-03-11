use std::collections::HashMap;

use super::User;


pub trait Command{}

pub struct CreateUser(String);
impl Command for CreateUser{}

pub struct DeleteUser(String);
impl Command for DeleteUser{}

pub struct CreditUser(String, u32);
impl Command for CreditUser{}

pub struct DebitUser(String, u32);
impl Command for DebitUser{}

pub type UserWriteRepository = HashMap<String, User>;


trait CommandHandler<C:Command, M>{
    fn handle(&mut self, cmd:C) -> Option<M>;
}

pub struct UserAggregate{
    repository:UserWriteRepository
}

impl UserAggregate {
    pub fn new() -> Self{
        UserAggregate { repository: HashMap::new() }
    }
}

impl CommandHandler<CreateUser, User> for UserAggregate{
    fn handle(&mut self, cmd:CreateUser) -> Option<User> {
        let user = User{ name:cmd.0, money: 0 };
        self.repository.insert(user.name.clone(), user.clone());
        Some(user)
    }
}

impl CommandHandler<DeleteUser, User> for UserAggregate {
    fn handle(&mut self, cmd:DeleteUser) -> Option<User> {
        self.repository.remove(&cmd.0)
    }
}

impl CommandHandler<CreditUser, User> for UserAggregate {
    fn handle(&mut self, cmd:CreditUser) -> Option<User> {
        
        if let Some(user) = self.repository.get(&cmd.0) {
            let credited = User{ money: user.money+cmd.1, ..user.clone()};
            self.repository.insert(user.name.clone(), credited.clone());
            Some(credited)
        }else{
            None
        }
    }
}

impl CommandHandler<DebitUser, User> for UserAggregate{
    fn handle(&mut self, cmd:DebitUser) -> Option<User> {
        if let Some(user) = self.repository.get(&cmd.0) {
            if let Some(value) = user.money.checked_sub(cmd.1){
                let debited = User{ money: value, ..user.clone()};
                self.repository.insert(user.name.clone(), debited.clone());
                return Some(debited);
            }

            None
            
            
        }else{
            None
        }
    }
}