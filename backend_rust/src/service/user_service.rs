use crate::{domain::user::User, repository::user_repository::UserRepository};

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        UserService { repository }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = self.repository.get_users().await?;
        Ok(users)
    }
}
