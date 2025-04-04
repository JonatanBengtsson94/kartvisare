use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::domain::{wms_details::WmsDetails, wms_group::WmsGroup, wms_summary::WmsSummary};

use super::repo_error::RepoError;

#[derive(Clone)]
pub struct PostgresWmsRepository {
    pool: PgPool,
}

impl PostgresWmsRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresWmsRepository { pool }
    }

    async fn get_subgroup(
        &self,
        parent_id: i32,
        user_id: i32,
    ) -> Result<Vec<WmsGroup>, sqlx::Error> {
        let group_query = "SELECT group_id, name FROM wms_groups WHERE parent_id = $1";
        let group_rows = sqlx::query(group_query)
            .bind(parent_id)
            .fetch_all(&self.pool)
            .await?;

        let mut wms_groups = Vec::new();
        for group_row in group_rows {
            let group_id: i32 = group_row.get("group_id");
            let group_name: String = group_row.get("name");

            let wms = self.get_wms_in_group(group_id, user_id).await?;
            let sub_groups = Box::pin(self.get_subgroup(group_id, user_id));
            let sub_groups = sub_groups.await?;

            wms_groups.push(WmsGroup {
                id: group_id,
                name: group_name,
                wms: Some(wms),
                sub_groups: Some(sub_groups),
            });
        }
        Ok(wms_groups)
    }

    async fn get_wms_in_group(
        &self,
        group_id: i32,
        user_id: i32,
    ) -> Result<Vec<WmsSummary>, sqlx::Error> {
        let query = r#"
            SELECT wms.wms_id, wms.name
            FROM wms
            INNER JOIN wms_user_groups_membership AS wugm
                ON wms.wms_id = wugm.wms_id
            INNER JOIN user_group_membership AS ugm
                ON ugm.group_id = wugm.group_id
            WHERE wms.group_id = $1
            AND ugm.user_id = $2
        "#;
        let rows = sqlx::query(query)
            .bind(group_id)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let wms: Vec<WmsSummary> = rows
            .into_iter()
            .map(|wms| WmsSummary {
                id: wms.get("wms_id"),
                name: wms.get("name"),
            })
            .collect();
        Ok(wms)
    }
}

#[async_trait]
pub trait WmsRepository {
    async fn get_wms_by_id(&self, wms_id: i32, user_id: i32) -> Result<WmsDetails, RepoError>;
    async fn add_wms(&self, wms_details: WmsDetails) -> Result<i32, RepoError>;
    async fn get_wms_groups(&self, user_id: i32) -> Result<Vec<WmsGroup>, RepoError>;
}

#[async_trait]
impl WmsRepository for PostgresWmsRepository {
    async fn get_wms_groups(&self, user_id: i32) -> Result<Vec<WmsGroup>, RepoError> {
        let query = "SELECT group_id, name FROM wms_groups WHERE parent_id IS NULL";
        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .map_err(RepoError::DatabaseError)?;

        let mut wms_groups = Vec::new();
        for row in rows {
            let group_id: i32 = row.get("group_id");
            let group_name: String = row.get("name");
            let wms = self
                .get_wms_in_group(group_id, user_id)
                .await
                .map_err(RepoError::DatabaseError)?;
            let sub_groups = self
                .get_subgroup(group_id, user_id)
                .await
                .map_err(RepoError::DatabaseError)?;

            wms_groups.push(WmsGroup {
                id: group_id,
                name: group_name,
                wms: Some(wms),
                sub_groups: Some(sub_groups),
            });
        }
        Ok(wms_groups)
    }

    async fn get_wms_by_id(&self, wms_id: i32, user_id: i32) -> Result<WmsDetails, RepoError> {
        let query = r#"
        SELECT wms.wms_id, wms.name, wms.description, wms.layers, wms.url, wms.version, wms.is_active, wms.auth_type, wms.auth_username, wms.auth_password
        FROM wms
        INNER JOIN wms_user_groups_membership AS wugm
            ON wms.wms_id = wugm.wms_id
        INNER JOIN user_group_membership AS ugm
            ON ugm.group_id = wugm.group_id
        WHERE wms.wms_id = $1
        AND ugm.user_id = $2
        "#;

        let row = sqlx::query(query)
            .bind(wms_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepoError::DatabaseError)?;

        match row {
            Some(r) => Ok(WmsDetails {
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
            }),
            None => {
                let query = "SELECT wms_id FROM wms WHERE wms_id = $1";
                let row = sqlx::query(query)
                    .bind(wms_id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(RepoError::DatabaseError)?;

                if row.is_some() {
                    return Err(RepoError::Forbidden);
                } else {
                    return Err(RepoError::NotFound);
                }
            }
        }
    }

    async fn add_wms(&self, wms_details: WmsDetails) -> Result<i32, RepoError> {
        let query = r#"
        INSERT INTO wms (name, description, layers, url, version, is_active, auth_type, auth_username, auth_password)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING wms_id
        "#;

        let result = sqlx::query(query)
            .bind(&wms_details.name)
            .bind(&wms_details.description)
            .bind(&wms_details.layers)
            .bind(&wms_details.url)
            .bind(&wms_details.version)
            .bind(&wms_details.is_active)
            .bind(&wms_details.auth_type)
            .bind(&wms_details.auth_username)
            .bind(&wms_details.auth_password)
            .fetch_one(&self.pool)
            .await
            .map_err(RepoError::DatabaseError)?;

        Ok(result.get("wms_id"))
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
        pool.execute("TRUNCATE table wms_user_groups_membership RESTART IDENTITY CASCADE")
            .await
            .unwrap();

        pool.execute("TRUNCATE table user_group_membership RESTART IDENTITY CASCADE")
            .await
            .unwrap();

        pool.execute("TRUNCATE TABLE wms RESTART IDENTITY CASCADE")
            .await
            .unwrap();

        pool.execute("TRUNCATE table wms_groups RESTART IDENTITY CASCADE")
            .await
            .unwrap();

        pool.execute("TRUNCATE table users RESTART IDENTITY CASCADE")
            .await
            .unwrap();

        pool.execute("TRUNCATE table user_groups RESTART IDENTITY CASCADE")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_wmsgroups() {
        let pool = setup_db().await;
        let repo = PostgresWmsRepository::new(pool.clone());

        sqlx::query(
            r#"
            INSERT INTO wms_groups (name) VALUES ('World')
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO wms_groups (name, parent_id) VALUES ('Usa', 1)
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO wms (name, layers, url, is_active, group_id) 
            VALUES ('States', ARRAY['topp:states'], 'http://localhost:8001/geoserver/wms', true, 2);
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO users (user_name) VALUES ('Bob');
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO user_groups (group_name) VALUES ('Admins');
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO user_group_membership (user_id, group_id) VALUES (1, 1);
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO wms_user_groups_membership (wms_id, group_id) VALUES (1, 1);
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        let wms_groups = repo.get_wms_groups(1).await.unwrap();

        assert_eq!(wms_groups.len(), 1);
        assert_eq!(wms_groups[0].name, "World");

        let sub_groups = wms_groups[0].sub_groups.as_ref().unwrap();
        assert_eq!(sub_groups.len(), 1);
        assert_eq!(sub_groups[0].name, "Usa");

        let wms = sub_groups[0].wms.as_ref().unwrap();
        assert_eq!(wms.len(), 1);
        assert_eq!(wms[0].name, "States");
    }

    #[tokio::test]
    async fn test_get_wms_by_id() {
        let pool = setup_db().await;
        let repo = PostgresWmsRepository::new(pool.clone());

        sqlx::query(
            r#"
            INSERT INTO users (user_name) VALUES ('Bob');
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO user_groups (group_name) VALUES ('Admins');
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO wms_groups (name) VALUES ('World')
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

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

        sqlx::query(
            r#"
            INSERT INTO user_group_membership (user_id, group_id) VALUES (1, 1);
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO wms_user_groups_membership (wms_id, group_id) VALUES (1, 1);
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        let details = repo.get_wms_by_id(1, 1).await.unwrap();

        assert_eq!(details.name, "States");
        assert_eq!(details.description.unwrap_or_default(), "usa population");
        assert_eq!(details.layers, vec!["topp:states"]);
        assert_eq!(details.url, "http://localhost:8001/geoserver/topp/wms");
        assert_eq!(details.version.unwrap_or_default(), "1.1.1");
        assert_eq!(details.is_active, true);
        assert_eq!(details.auth_type.unwrap_or_default(), "Basic");
        assert_eq!(details.auth_username.unwrap_or_default(), "username");
        assert_eq!(details.auth_password.unwrap_or_default(), "password");
    }

    #[tokio::test]
    async fn test_add_wms() {
        let pool = setup_db().await;
        let repo = PostgresWmsRepository::new(pool.clone());

        let wms_details = WmsDetails {
            id: None,
            name: "States".to_string(),
            description: None,
            layers: vec!["topp.states".to_string()],
            url: "http://localhost:8001/geoserver/topp/wms".to_string(),
            version: None,
            is_active: true,
            auth_type: None,
            auth_username: None,
            auth_password: None,
        };

        let inserted_id = repo.add_wms(wms_details).await.unwrap();

        assert!(inserted_id > 0, "Expected a valid wms_id to be returned");
    }
}
