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
