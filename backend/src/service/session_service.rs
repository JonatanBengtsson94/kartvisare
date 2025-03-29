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
