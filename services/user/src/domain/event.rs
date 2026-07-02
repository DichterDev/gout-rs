pub enum UserEventPayload {
    UserRegistered { name: String, password: String },
    UserRenamed { old_name: String, new_name: String },
    UserPasswordChanged,
    UserDeactivated,
}

pub struct UserEvent {
    pub user_id: String,
    pub payload: UserEventPayload,
}
