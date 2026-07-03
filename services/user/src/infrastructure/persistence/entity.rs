use sqlx::types::Uuid;

use crate::domain::aggregate::User;

#[derive(sqlx::FromRow)]
pub struct UserEntity {
    id: Uuid,
    name: String,
    password: String,
    is_active: bool,
}

impl From<&User> for UserEntity {
    fn from(user: &User) -> Self {
        Self {
            id: user.id(),
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
