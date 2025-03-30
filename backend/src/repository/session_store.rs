use crate::domain::session::Session;

use super::repo_error::RepoError;
use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SessionData {
    user_id: i32,
    is_admin: bool,
}

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
    async fn load_session(&self, session_id: &str) -> Result<Session, RepoError>;
    async fn save_session(&self, user_id: i32, is_admin: bool) -> Result<Session, RepoError>;
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn load_session(&self, session_id: &str) -> Result<Session, RepoError> {
        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(RepoError::RedisError)?;

        let session_data: Option<String> =
            con.get(session_id).await.map_err(RepoError::RedisError)?;
        match session_data {
            Some(data) => match serde_json::from_str::<SessionData>(&data) {
                Ok(session_data) => Ok(Session {
                    session_id: session_id.to_string(),
                    user_id: session_data.user_id,
                    is_admin: session_data.is_admin,
                }),
                Err(e) => Err(RepoError::SerializationError(e)),
            },
            None => Err(RepoError::NotFound),
        }
    }

    async fn save_session(&self, user_id: i32, is_admin: bool) -> Result<Session, RepoError> {
        let session_data = SessionData { user_id, is_admin };
        let session_json =
            serde_json::to_string(&session_data).map_err(RepoError::SerializationError)?;

        let session_id = uuid::Uuid::new_v4().to_string();

        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(RepoError::RedisError)?;

        let _: () = con
            .set(&session_id, &session_json)
            .await
            .map_err(RepoError::RedisError)?;

        Ok(Session {
            session_id,
            user_id,
            is_admin,
        })
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use redis::Client;
    use std::env;

    use crate::repository::{
        repo_error::RepoError,
        session_store::{RedisSessionStore, SessionStore},
    };

    async fn get_client() -> Client {
        dotenv().ok();
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in environment");
        let redis_client = Client::open(redis_url).expect("Invalid Redis URL");
        redis_client
    }

    #[tokio::test]
    async fn test_load_session_found() {
        let client = get_client().await;
        let session_store = RedisSessionStore::new(client);

        let session = session_store.save_session(1, true).await.unwrap();

        let result = session_store.load_session(&session.session_id).await;

        assert!(result.is_ok());
        let session = result.unwrap();
        assert_eq!(session.user_id, 1);
        assert!(session.is_admin);
    }

    #[tokio::test]
    async fn test_load_session_not_found() {
        let client = get_client().await;
        let session_store = RedisSessionStore::new(client);

        let result = session_store.load_session("non_existent_session_id").await;

        assert!(result.is_err());
        assert!(matches!(result, Err(RepoError::NotFound)));
    }

    #[tokio::test]
    async fn test_save_session() {
        let client = get_client().await;
        let session_store = RedisSessionStore::new(client);

        let result = session_store.save_session(1, true).await;

        assert!(result.is_ok());
        let session = result.unwrap();
        assert_eq!(session.user_id, 1);
        assert!(session.is_admin);
    }
}
