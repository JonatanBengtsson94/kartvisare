use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::domain::user::User;

use super::repo_error::RepoError;

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
    async fn is_admin(&self, user_id: i32) -> Result<bool, RepoError>;
    async fn get_users(&self) -> Result<Vec<User>, RepoError>;
    async fn add_user(&self, user: User) -> Result<(), RepoError>;
    async fn add_user_to_user_group(&self, user_id: i32, group_id: i32) -> Result<(), RepoError>;
    async fn add_user_group(&self, group_name: String) -> Result<(), RepoError>;
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn is_admin(&self, user_id: i32) -> Result<bool, RepoError> {
        let query = "SELECT user_id FROM user_group_membership WHERE user_id = $1 AND group_id = 1";
        let row = sqlx::query(query)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepoError::DatabaseError)?;

        Ok(row.is_some())
    }

    async fn get_users(&self) -> Result<Vec<User>, RepoError> {
        let query = r#"
            SELECT user_id, user_name, idp_id FROM users
        "#;

        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .map_err(RepoError::DatabaseError)?;

        let users: Vec<User> = rows
            .into_iter()
            .map(|row| User {
                id: row.get("user_id"),
                user_name: row.get("user_name"),
                idp_id: row.get("idp_id"),
            })
            .collect();
        Ok(users)
    }

    async fn add_user(&self, user: User) -> Result<(), RepoError> {
        let query = "INSERT INTO users (user_name, idp_id) VALUES ($1, $2)";
        sqlx::query(query)
            .bind(&user.user_name)
            .bind(&user.idp_id)
            .execute(&self.pool)
            .await
            .map_err(RepoError::DatabaseError)?;

        Ok(())
    }

    async fn add_user_to_user_group(&self, user_id: i32, group_id: i32) -> Result<(), RepoError> {
        let query = "INSERT INTO user_group_membership VALUES ($1, $2)";
        sqlx::query(query)
            .bind(user_id)
            .bind(group_id)
            .execute(&self.pool)
            .await
            .map_err(RepoError::DatabaseError)?;

        Ok(())
    }

    async fn add_user_group(&self, group_name: String) -> Result<(), RepoError> {
        let query = "INSERT INTO user_groups (group_name) VALUES ($1)";
        sqlx::query(query)
            .bind(group_name)
            .execute(&self.pool)
            .await
            .map_err(RepoError::DatabaseError)?;

        Ok(())
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

        clean_db(&pool).await;

        pool
    }

    async fn clean_db(pool: &PgPool) {
        pool.execute("TRUNCATE TABLE user_group_membership RESTART IDENTITY CASCADE")
            .await
            .unwrap();

        pool.execute("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
            .await
            .unwrap();

        pool.execute("TRUNCATE TABLE user_groups RESTART IDENTITY CASCADE")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_users() {
        let pool = setup_db().await;
        let repo = PostgresUserRepository::new(pool.clone());

        sqlx::query("INSERT INTO users (user_name, idp_id) VALUES ($1, $2), ($3, $4)")
            .bind("John")
            .bind("JohnIDP")
            .bind("Jane")
            .bind("JaneIDP")
            .execute(&pool)
            .await
            .unwrap();

        let users = repo.get_users().await.unwrap();

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].user_name, "John");
        assert_eq!(users[0].idp_id, "JohnIDP");
        assert_eq!(users[1].user_name, "Jane");
        assert_eq!(users[1].idp_id, "JaneIDP");
    }

    #[tokio::test]
    async fn test_is_admin() {
        let pool = setup_db().await;
        let repo = PostgresUserRepository::new(pool.clone());

        sqlx::query("INSERT into users (user_name) VALUES ($1)")
            .bind("Admin_User")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT into users (user_name) VALUES ($1)")
            .bind("Normal_User")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO user_groups (group_name) VALUES ($1)")
            .bind("Admin")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO user_group_membership (user_id, group_id) VALUES ($1, $2)")
            .bind(1)
            .bind(1)
            .execute(&pool)
            .await
            .unwrap();

        let admin = repo.is_admin(1).await.unwrap();
        assert_eq!(admin, true);
        let non_admin = repo.is_admin(2).await.unwrap();
        assert_eq!(non_admin, false);
    }

    #[tokio::test]
    async fn test_add_user() {
        let pool = setup_db().await;
        let repo = PostgresUserRepository::new(pool.clone());

        let user = User {
            id: None,
            user_name: "John".to_string(),
            idp_id: "JohnIDP".to_string(),
        };

        let added = repo.add_user(user).await.unwrap();
        assert_eq!(added, ());
    }

    #[tokio::test]
    async fn test_add_user_to_user_group() {
        let pool = setup_db().await;
        let repo = PostgresUserRepository::new(pool.clone());

        sqlx::query("INSERT into users (user_name) VALUES ($1)")
            .bind("Normal_User")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO user_groups (group_name) VALUES ($1)")
            .bind("Admin")
            .execute(&pool)
            .await
            .unwrap();

        let user_added = repo.add_user_to_user_group(1, 1).await.unwrap();
        assert_eq!(user_added, ());
    }

    #[tokio::test]
    async fn test_add_user_group() {
        let pool = setup_db().await;
        let repo = PostgresUserRepository::new(pool.clone());

        let group_name = "viewers".to_string();
        let added = repo.add_user_group(group_name).await.unwrap();
        assert_eq!(added, ());
    }
}
