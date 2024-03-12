use crate::events::Event;

use self::read::UserReadRepository;

pub mod read;
pub mod write;

pub struct UserProjector{
    pub repository:UserReadRepository
}

impl UserProjector {
    pub fn new() -> Self {
        UserProjector { repository: UserReadRepository::default() }
    }

    pub fn project(&mut self, event:Event) {
        self.apply(event);
        
    }

    fn apply(&mut self, event:Event) {
        match event {
            Event::UserCreatedEvent(_, user) => self.repository.add(user),
            Event::UserDeletedEvent(_, user) => self.repository.remove(&user.name),
            Event::UserCreditedEvent(_, mut user, amount) => {
                self.repository.remove(&user.name);
                user.money += amount as i32;
                
                self.repository.add(user);
            },
            Event::UserDebitedEvent(_, mut user, amount) => {
                self.repository.remove(&user.name);
                user.money -= amount as i32;
                
                self.repository.add(user);
            },
        }
    }
}