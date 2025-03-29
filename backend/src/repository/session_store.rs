use crate::domain::session::Session;

use super::repo_error::RepoError;
use async_trait::async_trait;
use redis::Client;

#[derive(Clone)]
pub struct RedisSessionStore {
    client: Client,
}

impl RedisSessionStore {
    pub fn new(client: Client) -> Self {
        RedisSessionStore { client }
    }
}

#[async_trait]
pub trait SessionStore {
    async fn load_session(&self, session_id: &str) -> Option<Session>;
    async fn save_session(&self, session_id: &str, session: &Session) -> Result<(), RepoError>;
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn load_session(&self, session_id: &str) -> Option<Session> {
        // TODO
        None
    }
    async fn save_session(&self, session_id: &str, session: &Session) -> Result<(), RepoError> {
        // TODO
        Ok(())
    }
}
