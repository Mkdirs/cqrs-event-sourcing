use self::read::UserReadRepository;

pub mod write;
pub mod read;

#[derive(Debug, Clone)]
pub struct Contact{
    pub mail:String
}

#[derive(Debug, Clone)]
pub struct User {
    pub name:String,
    pub money:i32,
    pub contact:Contact
}

pub type Users = Vec<User>;

pub struct UserProjector{
    repository:UserReadRepository
}

impl UserProjector {
    pub fn new(repository:UserReadRepository) -> Self {
        UserProjector { repository }
    }

    pub fn project(&mut self, user:User) {
        self.repository.add(user);
    }
}