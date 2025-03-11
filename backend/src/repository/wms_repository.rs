use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::domain::{wms_details::WmsDetails, wms_summary::WmsSummary};

#[derive(Clone)]
pub struct PostgresWmsRepository {
    pool: PgPool,
}

impl PostgresWmsRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresWmsRepository { pool }
    }
}

#[async_trait]
pub trait WmsRepository {
    async fn get_wms_summaries(&self) -> Result<Vec<WmsSummary>, sqlx::Error>;
    async fn get_wms_details(&self, id: i32) -> Result<Option<WmsDetails>, sqlx::Error>;
}

#[async_trait]
impl WmsRepository for PostgresWmsRepository {
    async fn get_wms_summaries(&self) -> Result<Vec<WmsSummary>, sqlx::Error> {
        let query = r#"
        SELECT wms_id, name FROM wms
        "#;

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let wms: Vec<WmsSummary> = rows
            .into_iter()
            .map(|wms| WmsSummary {
                id: wms.get("wms_id"),
                name: wms.get("name"),
            })
            .collect();
        Ok(wms)
    }

    async fn get_wms_details(&self, id: i32) -> Result<Option<WmsDetails>, sqlx::Error> {
        let query = r#"
        SELECT wms_id, name, description, layers, url, version, is_active, auth_type, auth_username, auth_password
        FROM wms WHERE wms_id = $1
        "#;

        let row = sqlx::query(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => Ok(Some(WmsDetails {
                id: r.get("wms_id"),
                name: r.get("name"),
                description: r.get("description"),
                layers: r.get("layers"),
                url: r.get("url"),
                version: r.get("version"),
                is_active: r.get("is_active"),
                auth_type: r.get("auth_type"),
                auth_username: r.get("auth_username"),
                auth_password: r.get("auth_password"),
            })),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Executor, PgPool};
    use std::env;

    async fn setup_db() -> PgPool {
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

        pool.execute("TRUNCATE TABLE wms RESTART IDENTITY CASCADE")
            .await
            .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_get_wms_summaries() {
        let pool = setup_db().await;
        let repo = PostgresWmsRepository::new(pool.clone());

        sqlx::query(
            r#"
            INSERT INTO wms (
                name,
                layers,
                url,
                is_active
            ) 
            VALUES 
            ($1, $2, $3, $4),
            ($5, $6, $7, $8)
            "#,
        )
        .bind("States") // $1
        .bind(vec!["topp:states".to_string()]) // $2
        .bind("http://localhost:8001/geoserver/topp/wms") // $3
        .bind(true) // $4
        .bind("Manhattan Roads") // $5
        .bind(vec!["tiger:tiger_roads".to_string()]) // $6
        .bind("http://localhost:8001/geoserver/tiger/wms") // $7
        .bind(true) // $8
        .execute(&pool)
        .await
        .unwrap();

        let summaries = repo.get_wms_summaries().await.unwrap();

        assert_eq!(summaries.len(), 2);
        assert_eq!(summaries[0].name, "States");
        assert_eq!(summaries[1].name, "Manhattan Roads");

        pool.execute("TRUNCATE TABLE wms RESTART IDENTITY CASCADE")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_wms_details() {
        let pool = setup_db().await;
        let repo = PostgresWmsRepository::new(pool.clone());

        sqlx::query(
            r#"
            INSERT INTO wms (
                name,
                description,
                layers,
                url,
                version,
                is_active,
                auth_type,
                auth_username,
                auth_password
            ) 
            VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind("States") // $1
        .bind("usa population") // $2
        .bind(vec!["topp:states".to_string()]) // $3
        .bind("http://localhost:8001/geoserver/topp/wms") // $4
        .bind("1.1.1") // $5
        .bind(true) // $6
        .bind("Basic") // $7
        .bind("username") // $8
        .bind("password") // $9
        .execute(&pool)
        .await
        .unwrap();

        let details = repo.get_wms_details(1).await.unwrap();

        assert!(details.is_some());
        let details = details.unwrap();
        assert_eq!(details.name, "States");
        assert_eq!(details.description.unwrap_or_default(), "usa population");
        assert_eq!(details.layers, vec!["topp:states"]);
        assert_eq!(details.url, "http://localhost:8001/geoserver/topp/wms");
        assert_eq!(details.version.unwrap_or_default(), "1.1.1");
        assert_eq!(details.is_active, true);
        assert_eq!(details.auth_type.unwrap_or_default(), "Basic");
        assert_eq!(details.auth_username.unwrap_or_default(), "username");
        assert_eq!(details.auth_password.unwrap_or_default(), "password");

        pool.execute("TRUNCATE TABLE wms RESTART IDENTITY CASCADE")
            .await
            .unwrap();
    }
}
