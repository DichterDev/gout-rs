use crate::domain::event::{UserEvent, UserEventPayload};

pub struct User {
    id: String,
    name: String,
    password: String,
    is_active: bool,
    events: Vec<UserEvent>,
}

impl User {
    pub fn rename(&mut self, name: String) {
        if self.name == name {
            return;
        }

        let old_name = std::mem::replace(&mut self.name, name.clone());

        let event = UserEvent {
            user_id: self.id.clone(),
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
            user_id: self.id.clone(),
            payload: UserEventPayload::UserDeactivated,
        };
        self.events.push(event);
    }

    pub fn pull_events(&mut self) -> Vec<UserEvent> {
        std::mem::take(&mut self.events)
    }
}
