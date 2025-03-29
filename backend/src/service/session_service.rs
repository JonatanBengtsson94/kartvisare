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

    pub async fn create_session(&self, user_id: i32) -> Result<Session, RepoError> {
        // TODO: Create a session in the store
        Ok(Session {
            user_id: 1,
            session_id: "abc".to_string(),
            is_admin: false,
        })
    }

    pub async fn get_session(&self, session_id: &str) -> Result<Session, RepoError> {
        // TODO: Get user id for session from the store
        Ok(Session {
            user_id: 1,
            session_id: "abc".to_string(),
            is_admin: false,
        })
    }
}
