use crate::{domain::{Contact, User}, events::{BaseEvent, Event, EventStore, UserUtility}};



pub trait Command{}

pub struct CreateUser(pub String, pub Contact);
impl Command for CreateUser{}

pub struct DeleteUser(pub String);
impl Command for DeleteUser{}

pub struct CreditUser(pub String, pub u32);
impl Command for CreditUser{}

pub struct DebitUser(pub String, pub u32);
impl Command for DebitUser{}


pub trait CommandHandler<C:Command>{
    fn handle(&mut self, cmd:C) -> Option<Event>;
}

pub struct UserAggregate{
    repository:EventStore
}

impl UserAggregate {
    pub fn new() -> Self{
        UserAggregate { repository: EventStore::default() }
    }
}

impl CommandHandler<CreateUser> for UserAggregate{
    fn handle(&mut self, cmd:CreateUser) -> Option<Event> {

        if self.repository.has(&cmd.0){
            return None;
        }
        
        let user = User{ name:cmd.0.clone(), contact: cmd.1, money: 0 };
        let event = Event::UserCreatedEvent(BaseEvent::default(), user);
        self.repository.add(cmd.0, event.clone());
        
        Some(event)
    }
}

impl CommandHandler<DeleteUser> for UserAggregate {
    fn handle(&mut self, cmd:DeleteUser) -> Option<Event> {
        if let Some(user) = UserUtility::recreate_state(&self.repository, &cmd.0) {
            let event = Event::UserDeletedEvent(BaseEvent::default(), user);
            self.repository.add(cmd.0, event.clone());
            return Some(event);
        }
        
        None
    }
}

impl CommandHandler<CreditUser> for UserAggregate {
    fn handle(&mut self, cmd:CreditUser) -> Option<Event> {
        
        if let Some(user) = UserUtility::recreate_state(&self.repository, &cmd.0) {
            let event = Event::UserCreditedEvent(BaseEvent::default(), user, cmd.1);
            self.repository.add(cmd.0, event.clone());

            Some(event)
        }else{
            None
        }
    }
}

impl CommandHandler<DebitUser> for UserAggregate{
    fn handle(&mut self, cmd:DebitUser) -> Option<Event> {
        if let Some(user) = UserUtility::recreate_state(&self.repository, &cmd.0) {
            let event = Event::UserDebitedEvent(BaseEvent::default(), user, cmd.1);
            self.repository.add(cmd.0, event.clone());

            Some(event)
        }else{
            None
        }
    }
}