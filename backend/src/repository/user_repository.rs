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

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use sqlx::{Executor, PgPool};
    use std::env;

    async fn setup_db() -> PgPool {
        dotenv().ok();
        let db_host = env::var("DB_HOST").expect("DB_HOST must be set in the environment");
        let db_port = env::var("DB_PORT").expect("DB_PORT must be set in the environment");
        let db_user = env::var("DB_USER").expect("DB_USER must be set in the environment");
        let db_password =
            env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in the environment");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set in the environment");

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        );

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to database");

        pool.execute("DELETE FROM users").await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_get_users() {
        let pool = setup_db().await;
        let repo = PostgresUserRepository::new(pool.clone());

        sqlx::query("INSERT INTO users (username) VALUES ($1), ($2)")
            .bind("John")
            .bind("Jane")
            .execute(&pool)
            .await
            .unwrap();

        let users = repo.get_users().await.unwrap();

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].user_name, "John");
        assert_eq!(users[1].user_name, "Jane");
    }
}
