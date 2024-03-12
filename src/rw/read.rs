use crate::domain::{User, Users};



trait Query{}

struct UsersQuery();
impl Query for UsersQuery{}

struct UsersWithMoneyBelow(i32);
impl Query for UsersWithMoneyBelow{}

struct UsersWithMoneyAbove(i32);
impl Query for UsersWithMoneyAbove{}

struct UsersInRedQuery();
impl Query for UsersInRedQuery{}


#[derive(Debug, Default)]
pub struct UserReadRepository {
    pub(self) users:Users
}

trait QueryHandler<Q:Query, R> {
    fn handle(&self, query:Q) -> R;
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

pub struct UserProjection {
    repository:UserReadRepository
}

impl UserProjection {
    pub fn new(repository: UserReadRepository) -> Self{
        UserProjection { repository }
    }
}

impl QueryHandler<UsersQuery, Users> for UserProjection{
    fn handle(&self, _:UsersQuery) -> Users {
        self.repository.users().to_vec()
    }
}

impl QueryHandler<UsersWithMoneyAbove, Users> for UserProjection {
    fn handle(&self, query:UsersWithMoneyAbove) -> Users {
        self.repository.users().to_vec().into_iter()
            .filter(|user| user.money >= query.0)
            .collect()
    }
}

impl QueryHandler<UsersWithMoneyBelow, Users> for UserProjection {
    fn handle(&self, query:UsersWithMoneyBelow) -> Users {
        self.repository.users().to_vec().into_iter()
            .filter(|user| user.money <= query.0)
            .collect()
    }
}

impl QueryHandler<UsersInRedQuery, Users> for UserProjection {
    fn handle(&self, _:UsersInRedQuery) -> Users {
        self.handle(UsersWithMoneyBelow(0))
    }
}