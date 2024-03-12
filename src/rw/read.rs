use crate::domain::{User, Users};



pub enum Query {
    UsersQuery,
    UsersWithMoneyBelow(i32),
    UsersWithMoneyAbove(i32),

}



#[derive(Debug, Default)]
pub struct UserReadRepository {
    pub(self) users:Users
}

impl UserReadRepository {

    pub fn add(&mut self, user:User){
        self.users.push(user);
    }

    pub fn remove(&mut self, name:&str){
        self.users = self.users.clone().into_iter().filter(|user| user.name != name).collect();
    }

    pub fn users(&self) -> &[User] {
        &self.users
    }

    
}

pub struct UserProjection();

impl UserProjection {

    pub fn handle(query:Query, repository: &UserReadRepository) -> Users {
        match query {
            Query::UsersQuery => repository.users().to_vec(),
            Query::UsersWithMoneyBelow(amount) => {
                repository.users().to_vec().into_iter()
                    .filter(|user| user.money <= amount)
                    .collect()
            },
            Query::UsersWithMoneyAbove(amount) => {
                repository.users().to_vec().into_iter()
                    .filter(|user| user.money >= amount)
                    .collect()
            }
        }
    }
}
