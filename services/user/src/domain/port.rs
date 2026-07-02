use crate::domain::aggregate::User;

pub enum RepositoryError {
    NotFound(String),
}

pub trait UserRepository {
    fn get_by_id(&self, id: String) -> Result<Option<User>, RepositoryError>;
    fn save(&self, user: &User) -> Result<(), RepositoryError>;
}
