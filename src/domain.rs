
#[derive(Debug, Clone, Default)]
pub struct Contact{
    pub mail:String
}

#[derive(Debug, Clone, Default)]
pub struct User {
    pub name:String,
    pub money:i32,
    pub contact:Contact
}

pub type Users = Vec<User>;