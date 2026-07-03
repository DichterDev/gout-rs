use sqlx::PgPool;

use crate::{
    domain::{
        aggregate::User,
        port::{RepositoryError, UserRepository},
    },
    infrastructure::persistence::entity::UserEntity,
};

pub struct SqlxUserRepository {
    pool: PgPool,
}

impl UserRepository for SqlxUserRepository {
    async fn get_by_id(&self, id: String) -> Result<Option<User>, RepositoryError> {
        todo!()
    }

    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        let entity = UserEntity::from(user);

        Ok(())
    }
}
