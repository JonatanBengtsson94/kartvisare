use crate::{
    domain::{wms_details::WmsDetails, wms_group::WmsGroup, wms_summary::WmsSummary},
    repository::wms_repository::WmsRepository,
};

#[derive(Clone)]
pub struct WmsService<R: WmsRepository> {
    repository: R,
}

impl<R: WmsRepository> WmsService<R> {
    pub fn new(repository: R) -> Self {
        WmsService { repository }
    }

    pub async fn get_wms_summaries(&self) -> Result<Vec<WmsSummary>, sqlx::Error> {
        let wms = self.repository.get_wms_summaries().await?;
        Ok(wms)
    }

    pub async fn get_wms_details(&self, id: i32) -> Result<Option<WmsDetails>, sqlx::Error> {
        let wms = self.repository.get_wms_details(id).await?;
        Ok(wms)
    }

    pub async fn add_wms(&self, wms_details: WmsDetails) -> Result<i32, sqlx::Error> {
        let inserted_id = self.repository.add_wms(wms_details).await?;
        Ok(inserted_id)
    }

    pub async fn get_wms_groups(&self) -> Result<Vec<WmsGroup>, sqlx::Error> {
        let wms_groups = self.repository.get_wms_groups().await?;
        Ok(wms_groups)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::wms_group::WmsGroup;

    use super::*;
    use async_trait::async_trait;
    use mockall::{mock, predicate::*};

    mock! {
        pub WmsRepositoryMock {}

        #[async_trait]
        impl WmsRepository for WmsRepositoryMock {
            async fn get_wms_summaries(&self) -> Result<Vec<WmsSummary>, sqlx::Error>;
            async fn get_wms_details(&self, id: i32) -> Result<Option<WmsDetails>, sqlx::Error>;
            async fn add_wms(&self, wms_details: WmsDetails) -> Result<i32, sqlx::Error>;
            async fn get_wms_groups(&self) -> Result<Vec<WmsGroup>, sqlx::Error>;
        }
    }

    #[tokio::test]
    async fn test_get_wms_summaries() {
        let mut mock_repo = MockWmsRepositoryMock::new();

        mock_repo.expect_get_wms_summaries().returning(|| {
            Ok(vec![
                WmsSummary {
                    id: 1,
                    name: "States".to_string(),
                },
                WmsSummary {
                    id: 2,
                    name: "Manhattan Roads".to_string(),
                },
            ])
        });

        let service = WmsService::new(mock_repo);

        let summaries = service.get_wms_summaries().await.unwrap();

        assert_eq!(summaries.len(), 2);
        assert_eq!(summaries[0].name, "States");
        assert_eq!(summaries[1].name, "Manhattan Roads");
    }

    #[tokio::test]
    async fn test_get_wms_details_found() {
        let mut mock_repo = MockWmsRepositoryMock::new();

        mock_repo
            .expect_get_wms_details()
            .with(eq(1))
            .returning(|_| {
                Ok(Some(WmsDetails {
                    id: Some(1),
                    name: "States".to_string(),
                    description: Some("usa population".to_string()),
                    layers: vec!["topp:states".to_string()],
                    url: "http://localhost:8001/geoserver/topp/wms".to_string(),
                    version: Some("1.1.1".to_string()),
                    is_active: true,
                    auth_type: Some("Basic".to_string()),
                    auth_username: Some("username".to_string()),
                    auth_password: Some("password".to_string()),
                }))
            });

        let service = WmsService::new(mock_repo);

        let details = service.get_wms_details(1).await.unwrap();

        assert!(details.is_some());
        let details = details.unwrap();
        assert_eq!(details.name, "States");
        assert_eq!(details.description.unwrap(), "usa population");
        assert_eq!(details.layers, vec!["topp:states"]);
    }

    #[tokio::test]
    async fn test_get_details_not_found() {
        let mut mock_repo = MockWmsRepositoryMock::new();

        mock_repo
            .expect_get_wms_details()
            .with(eq(999))
            .returning(|_| Ok(None));

        let service = WmsService::new(mock_repo);

        let details = service.get_wms_details(999).await.unwrap();
        assert!(details.is_none());
    }

    #[tokio::test]
    async fn test_add_wms() {
        let mut mock_repo = MockWmsRepositoryMock::new();

        mock_repo
            .expect_add_wms()
            .with(eq(WmsDetails {
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
            }))
            .returning(|_| Ok(123));

        let service = WmsService::new(mock_repo);

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

        let result = service.add_wms(wms_details).await;
        assert_eq!(result.unwrap(), 123);
    }

    #[tokio::test]
    async fn test_get_wms_groups() {
        let mut mock_repo = MockWmsRepositoryMock::new();

        mock_repo.expect_get_wms_groups().returning(|| {
            Ok(vec![WmsGroup {
                id: 1,
                name: "test".to_string(),
                wms: None,
                sub_groups: None,
            }])
        });

        let service = WmsService::new(mock_repo);

        let wms_group = service.get_wms_groups().await.unwrap();
        assert_eq!(wms_group.len(), 1);
        assert_eq!(wms_group[0].name, "test");
    }
}
