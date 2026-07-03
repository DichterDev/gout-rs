use crate::domain::aggregate::User;

pub struct UserEntity {
    id: String,
    name: String,
    password: String,
    is_active: bool,
}

impl From<&User> for UserEntity {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().to_string(),
            name: user.name().to_string(),
            password: user.password().to_string(),
            is_active: user.is_active(),
        }
    }
}

impl From<UserEntity> for User {
    fn from(user: UserEntity) -> Self {
        User::reconstruct(user.id, user.name, user.password, user.is_active)
    }
}
