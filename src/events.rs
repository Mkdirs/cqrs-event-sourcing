use std::{collections::HashMap, fmt::Debug, time::Instant};

use uuid::Uuid;

use crate::domain::User;

#[derive(Debug, Clone)]
struct BaseEvent {
    pub id:Uuid,
    pub created: Instant
}

impl Default for BaseEvent{
    fn default() -> Self {
        BaseEvent { id: Uuid::new_v4(), created: Instant::now() }
    }
}

#[derive(Debug, Clone)]
enum Event {
    UserCreatedEvent(BaseEvent, User),
    UserDeletedEvent(BaseEvent, User),
    UserCreditedEvent(BaseEvent, User, u32),
    UserDebitedEvent(BaseEvent, User, u32)
}


#[derive(Debug, Default)]
struct EventStore {
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
}

struct UserUtility ();

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

pub struct UserService {
    repository: EventStore
}

impl UserService {
    pub fn new(repository:EventStore) -> Self{
        UserService { repository }
    }

    pub fn create_user(&mut self, user:User) {
        let base = BaseEvent::default();
        self.repository.add(user.name.clone(), Event::UserCreatedEvent(base, user));
    }

    pub fn delete_user(&mut self, name:String) {
        if let Some(user) = UserUtility::recreate_state(&self.repository, &name){
            self.repository.add(name, Event::UserDeletedEvent(BaseEvent::default(), user));
        }
    }

    pub fn credit_user(&mut self, name:String, amount:u32) {
        if let Some(user) = UserUtility::recreate_state(&self.repository, &name){
            self.repository.add(name, Event::UserCreditedEvent(BaseEvent::default(), user, amount));
        }
    }

    pub fn debit_user(&mut self, name:String, amount:u32) {
        if let Some(user) = UserUtility::recreate_state(&self.repository, &name){
            self.repository.add(name, Event::UserDebitedEvent(BaseEvent::default(), user, amount));
        }
    }
}