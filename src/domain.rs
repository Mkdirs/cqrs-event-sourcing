use std::fmt::Display;


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

impl Display for User{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, <{}>, {}€", self.name, self.contact.mail, self.money))
    }
}

pub type Users = Vec<User>;