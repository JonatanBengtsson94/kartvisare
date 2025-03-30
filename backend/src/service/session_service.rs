use crate::{
    domain::session::Session,
    repository::{repo_error::RepoError, session_store::SessionStore},
};

#[derive(Clone)]
pub struct SessionService<S: SessionStore> {
    store: S,
}

impl<S: SessionStore> SessionService<S> {
    pub fn new(store: S) -> Self {
        SessionService { store }
    }

    pub async fn create_session(&self, user_id: i32, is_admin: bool) -> Result<Session, RepoError> {
        let session = self.store.save_session(user_id, is_admin).await?;
        Ok(session)
    }

    pub async fn get_session(&self, session_id: &str) -> Result<Session, RepoError> {
        let session = self.store.load_session(session_id).await?;
        Ok(session)
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        domain::session::Session,
        repository::{repo_error::RepoError, session_store::SessionStore},
        service::session_service::SessionService,
    };
    use async_trait::async_trait;
    use mockall::{mock, predicate::*};

    mock! {
        pub SessionStoreMock {}
        #[async_trait]
        impl SessionStore for SessionStoreMock {
            async fn load_session(&self, session_id: &str) -> Result<Session, RepoError>;
            async fn save_session(&self, user_id: i32, is_admin: bool) -> Result<Session, RepoError>;
        }
    }

    #[tokio::test]
    async fn test_create_session_success() {
        let mut mock_store = MockSessionStoreMock::new();
        mock_store
            .expect_save_session()
            .with(eq(1), eq(true))
            .returning(|_, _| {
                Ok(Session {
                    session_id: "session123".to_string(),
                    user_id: 1,
                    is_admin: true,
                })
            });

        let service = SessionService::new(mock_store);
        let session = service.create_session(1, true).await.unwrap();

        assert_eq!(session.user_id, 1);
        assert_eq!(session.is_admin, true);
    }

    #[tokio::test]
    async fn test_get_session_success() {
        let mut mock_store = MockSessionStoreMock::new();

        mock_store
            .expect_load_session()
            .with(eq("session123"))
            .returning(|_| {
                Ok(Session {
                    session_id: "session123".to_string(),
                    user_id: 2,
                    is_admin: false,
                })
            });

        let service = SessionService::new(mock_store);
        let session = service.get_session("session123").await.unwrap();

        assert_eq!(session.user_id, 2);
        assert_eq!(session.is_admin, false);
    }

    #[tokio::test]
    async fn test_get_session_not_found() {
        let mut mock_store = MockSessionStoreMock::new();

        mock_store
            .expect_load_session()
            .with(eq("session123"))
            .returning(|_| Err(RepoError::NotFound));

        let service = SessionService::new(mock_store);
        let result = service.get_session("session123").await;

        assert!(result.is_err());
        assert!(matches!(result, Err(RepoError::NotFound)));
    }
}
