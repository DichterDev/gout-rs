use sqlx::types::Uuid;

use crate::domain::event::{UserEvent, UserEventPayload};

pub struct User {
    id: Uuid,
    name: String,
    password: String,
    is_active: bool,
    events: Vec<UserEvent>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            password: Default::default(),
            is_active: Default::default(),
            events: Default::default(),
        }
    }
}

impl User {
    pub fn reconstruct(id: Uuid, name: String, password: String, is_active: bool) -> User {
        Self {
            id,
            name,
            password,
            is_active,
            ..Default::default()
        }
    }

    pub fn rename(&mut self, name: String) {
        if self.name == name {
            return;
        }

        let old_name = std::mem::replace(&mut self.name, name.clone());

        let event = UserEvent {
            user_id: self.id.to_string(),
            payload: UserEventPayload::UserRenamed {
                old_name,
                new_name: name,
            },
        };

        self.events.push(event)
    }

    pub fn deactivate(&mut self) {
        if !self.is_active {
            return;
        }

        self.is_active = false;

        let event = UserEvent {
            user_id: self.id.to_string(),
            payload: UserEventPayload::UserDeactivated,
        };
        self.events.push(event);
    }

    pub fn pull_events(&mut self) -> Vec<UserEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
