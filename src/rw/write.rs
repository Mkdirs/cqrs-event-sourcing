use crate::{domain::{Contact, User}, events::{BaseEvent, Event, EventStore, UserUtility}};



pub trait Command{}

pub struct CreateUser(String, Contact);
impl Command for CreateUser{}

pub struct DeleteUser(String);
impl Command for DeleteUser{}

pub struct CreditUser(String, u32);
impl Command for CreditUser{}

pub struct DebitUser(String, u32);
impl Command for DebitUser{}


trait CommandHandler<C:Command>{
    fn handle(&mut self, cmd:C) -> Vec<Event>;
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
    fn handle(&mut self, cmd:CreateUser) -> Vec<Event> {
        let user = User{ name:cmd.0.clone(), contact: cmd.1, money: 0 };
        let event = Event::UserCreatedEvent(BaseEvent::default(), user);
        self.repository.add(cmd.0, event.clone());
        
        vec![event]
    }
}

impl CommandHandler<DeleteUser> for UserAggregate {
    fn handle(&mut self, cmd:DeleteUser) -> Vec<Event> {
        if let Some(user) = UserUtility::recreate_state(&self.repository, &cmd.0) {
            let event = Event::UserDeletedEvent(BaseEvent::default(), user);
            self.repository.add(cmd.0, event.clone());
            return vec![event];
        }
        
        vec![]
    }
}

impl CommandHandler<CreditUser> for UserAggregate {
    fn handle(&mut self, cmd:CreditUser) -> Vec<Event> {
        
        if let Some(user) = UserUtility::recreate_state(&self.repository, &cmd.0) {
            let event = Event::UserCreditedEvent(BaseEvent::default(), user, cmd.1);
            self.repository.add(cmd.0, event.clone());

            vec![event]
        }else{
            vec![]
        }
    }
}

impl CommandHandler<DebitUser> for UserAggregate{
    fn handle(&mut self, cmd:DebitUser) -> Vec<Event> {
        if let Some(user) = UserUtility::recreate_state(&self.repository, &cmd.0) {
            let event = Event::UserDebitedEvent(BaseEvent::default(), user, cmd.1);
            self.repository.add(cmd.0, event.clone());

            vec![event]
        }else{
            vec![]
        }
    }
}