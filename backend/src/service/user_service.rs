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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::mock;

    mock! {
        pub UserRepositoryMock {}

        #[async_trait]
        impl UserRepository for UserRepositoryMock {
            async fn get_users(&self) -> Result<Vec<User>, sqlx::Error>;
        }
    }

    #[tokio::test]
    async fn test_get_users() {
        let mut mock_repo = MockUserRepositoryMock::new();

        mock_repo.expect_get_users().returning(|| {
            Ok(vec![
                User {
                    id: 1,
                    user_name: "John".to_string(),
                },
                User {
                    id: 2,
                    user_name: "Jane".to_string(),
                },
            ])
        });

        let service = UserService::new(mock_repo);

        let users = service.get_users().await.unwrap();

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].user_name, "John");
        assert_eq!(users[1].user_name, "Jane");
    }
}
