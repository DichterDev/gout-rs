use crate::domain::aggregate::User;

pub enum RepositoryError {
    NotFound(String),
}

pub trait UserRepository {
    async fn get_by_id(&self, id: String) -> Result<Option<User>, RepositoryError>;
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
}
