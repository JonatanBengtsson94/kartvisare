use crate::{
    domain::user::User,
    repository::{repo_error::RepoError, user_repository::UserRepository},
};

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        UserService { repository }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, RepoError> {
        let users = self.repository.get_users().await?;
        Ok(users)
    }

    pub async fn add_user(&self, user: User) -> Result<(), RepoError> {
        self.repository.add_user(user).await?;
        Ok(())
    }

    pub async fn add_user_to_group(&self, user_id: i32, group_id: i32) -> Result<(), RepoError> {
        self.repository
            .add_user_to_user_group(user_id, group_id)
            .await?;
        Ok(())
    }

    pub async fn add_group(&self, group_name: String) -> Result<(), RepoError> {
        self.repository.add_user_group(group_name).await?;
        Ok(())
    }

    pub async fn is_admin(&self, user_id: i32) -> Result<bool, RepoError> {
        let is_admin = self.repository.is_admin(user_id).await?;
        Ok(is_admin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::{
        mock,
        predicate::{self, *},
    };

    mock! {
        pub UserRepositoryMock {}

        #[async_trait]
        impl UserRepository for UserRepositoryMock {
            async fn is_admin(&self, user_id: i32) -> Result<bool, RepoError>;
            async fn get_users(&self) -> Result<Vec<User>, RepoError>;
            async fn add_user(&self, user: User) -> Result<(), RepoError>;
            async fn add_user_to_user_group(&self, user_id: i32, group_id: i32) -> Result<(), RepoError>;
            async fn add_user_group(&self, group_name: String) -> Result<(), RepoError>;
        }
    }

    #[tokio::test]
    async fn test_add_user() {
        let mut mock_repo = MockUserRepositoryMock::new();
        let mock_user = User {
            id: None,
            user_name: "Mock".to_string(),
            idp_id: "MockIDP".to_string(),
        };

        mock_repo
            .expect_add_user()
            .with(predicate::function(|u: &User| {
                u.user_name == "Mock" && u.idp_id == "MockIDP"
            }))
            .returning(|_| Ok(()));

        let service = UserService::new(mock_repo);
        let added = service.add_user(mock_user).await.unwrap();

        assert_eq!(added, ());
    }

    #[tokio::test]
    async fn test_add_group() {
        let mut mock_repo = MockUserRepositoryMock::new();
        mock_repo
            .expect_add_user_group()
            .with(eq("MockGroup".to_string()))
            .returning(|_| Ok(()));

        let service = UserService::new(mock_repo);
        let added = service.add_group("MockGroup".to_string()).await.unwrap();

        assert_eq!(added, ());
    }

    #[tokio::test]
    async fn test_add_user_to_group() {
        let mut mock_repo = MockUserRepositoryMock::new();
        mock_repo
            .expect_add_user_to_user_group()
            .with(eq(1), eq(1))
            .returning(|_, _| Ok(()));

        let service = UserService::new(mock_repo);
        let user_added_to_group = service.add_user_to_group(1, 1).await.unwrap();

        assert_eq!(user_added_to_group, ());
    }

    #[tokio::test]
    async fn test_get_users() {
        let mut mock_repo = MockUserRepositoryMock::new();

        mock_repo.expect_get_users().returning(|| {
            Ok(vec![
                User {
                    id: None,
                    user_name: "John".to_string(),
                    idp_id: "JohnIDP".to_string(),
                },
                User {
                    id: None,
                    user_name: "Jane".to_string(),
                    idp_id: "JaneIDP".to_string(),
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
