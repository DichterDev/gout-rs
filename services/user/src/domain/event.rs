enum UserEventPayload {
    UserLoggedIn,
    UserRegistered { name: String },
    UserRenamed { old_name: String, new_name: String },
    UserPasswordChanged,
    UserDeactivated,
}

struct UserEvent {
    user_id: String,
    payload: UserEventPayload,
}
