use std::{collections::HashMap, fmt::Debug, time::Instant};

use uuid::Uuid;

use crate::domain::User;

#[derive(Debug, Clone)]
pub struct BaseEvent {
    pub id:Uuid,
    pub created: Instant
}

impl Default for BaseEvent{
    fn default() -> Self {
        BaseEvent { id: Uuid::new_v4(), created: Instant::now() }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    UserCreatedEvent(BaseEvent, User),
    UserDeletedEvent(BaseEvent, User),
    UserCreditedEvent(BaseEvent, User, u32),
    UserDebitedEvent(BaseEvent, User, u32)
}


#[derive(Debug, Default)]
pub struct EventStore {
    store: HashMap<String, Vec<Event>>
}

impl EventStore {
    pub fn add(&mut self, id:String, event: Event){

        let mut events = vec![];
        if let Some(e) = self.store.get(&id){
            events = e.to_vec();
        }
        events.push(event);

        self.store.insert(id, events);
    }

    pub fn has(&self, id:&str) -> bool {
        self.store.contains_key(id)
    }
}

pub struct UserUtility ();

impl UserUtility {
    pub fn recreate_state(repository:&EventStore, name:&str) -> Option<User> {
        let mut user = User::default();
        if let Some(events) = repository.store.get(name){
            for event in events {
                match event {
                    Event::UserCreatedEvent(_, usr) => user = usr.clone(),
                    Event::UserDeletedEvent(_, _) => return None,
                    Event::UserCreditedEvent(_, _, amount) => user.money += *amount as i32,
                    Event::UserDebitedEvent(_, _, amount) => user.money -= *amount as i32,
                }
            }

            return Some(user);
        }

        None
    }
}