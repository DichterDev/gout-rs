use crate::domain::event::{self, UserEvent, UserEventPayload};

struct NewUser {
    name: String,
    password: String,
}

struct User {
    id: String,
    name: String,
    password: String,
    events: Vec<UserEvent>,
}

struct DeactivatedUser {
    id: String,
    name: String,
    password: String,
}

impl User {
    pub fn rename(&mut self, name: String) {
        let old_name = std::mem::replace(&mut self.name, name.clone());

        if old_name == name {
            return;
        }

        let event = UserEvent {
            user_id: self.id.clone(),
            payload: UserEventPayload::UserRenamed {
                old_name,
                new_name: name,
            },
        };

        self.events.push(event)
    }

    pub fn deactivate(mut self) -> (DeactivatedUser, Vec<UserEvent>) {
        let event = UserEvent {
            user_id: self.id.clone(),
            payload: UserEventPayload::UserDeactivated,
        };

        let user = DeactivatedUser {
            id: self.id,
            name: self.name,
            password: self.password,
        };

        self.events.push(event);

        (user, self.events)
    }
}
