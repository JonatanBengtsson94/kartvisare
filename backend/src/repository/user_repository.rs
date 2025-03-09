use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::domain::user::User;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresUserRepository { pool }
    }
}

#[async_trait]
pub trait UserRepository {
    async fn get_users(&self) -> Result<Vec<User>, sqlx::Error>;
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let query = r#"
            SELECT user_id, username FROM users
        "#;

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let users: Vec<User> = rows
            .into_iter()
            .map(|row| User {
                id: row.get("user_id"),
                user_name: row.get("username"),
            })
            .collect();
        Ok(users)
    }
}
