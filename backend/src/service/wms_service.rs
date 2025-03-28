use crate::{
    domain::{wms_details::WmsDetails, wms_group::WmsGroup},
    repository::{repo_error::RepoError, wms_repository::WmsRepository},
};

#[derive(Clone)]
pub struct WmsService<R: WmsRepository> {
    repository: R,
}

impl<R: WmsRepository> WmsService<R> {
    pub fn new(repository: R) -> Self {
        WmsService { repository }
    }

    pub async fn get_wms_by_id(&self, wms_id: i32, user_id: i32) -> Result<WmsDetails, RepoError> {
        self.repository.get_wms_by_id(wms_id, user_id).await
    }

    pub async fn add_wms(&self, wms_details: WmsDetails) -> Result<i32, RepoError> {
        let inserted_id = self.repository.add_wms(wms_details).await?;
        Ok(inserted_id)
    }

    pub async fn get_wms_groups(&self, user_id: i32) -> Result<Vec<WmsGroup>, RepoError> {
        let wms_groups = self.repository.get_wms_groups(user_id).await?;
        Ok(wms_groups)
    }
}

#[cfg(test)]
mod tests {
    use crate::{domain::wms_group::WmsGroup, repository::repo_error::RepoError};

    use super::*;
    use async_trait::async_trait;
    use mockall::{mock, predicate::*};

    mock! {
        pub WmsRepositoryMock {}

        #[async_trait]
        impl WmsRepository for WmsRepositoryMock {
            async fn get_wms_by_id(&self, wms_id: i32, user_id: i32) -> Result<WmsDetails, RepoError>;
            async fn add_wms(&self, wms_details: WmsDetails) -> Result<i32, RepoError>;
            async fn get_wms_groups(&self, user_id: i32) -> Result<Vec<WmsGroup>, RepoError>;
        }
    }

    #[tokio::test]
    async fn test_get_wms_by_id() {
        let mut mock_repo = MockWmsRepositoryMock::new();

        mock_repo
            .expect_get_wms_by_id()
            .with(eq(1), eq(1))
            .returning(|_, _| {
                Ok(WmsDetails {
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
                })
            });

        mock_repo
            .expect_get_wms_by_id()
            .with(eq(999), eq(1))
            .returning(|_, _| Err(RepoError::NotFound));

        mock_repo
            .expect_get_wms_by_id()
            .with(eq(1), eq(2))
            .returning(|_, _| Err(RepoError::Forbidden));

        let service = WmsService::new(mock_repo);

        let details = service.get_wms_by_id(1, 1).await.unwrap();

        assert_eq!(details.name, "States");
        assert_eq!(details.description.unwrap(), "usa population");
        assert_eq!(details.layers, vec!["topp:states"]);

        let result = service.get_wms_by_id(999, 1).await;
        assert!(matches!(result, Err(RepoError::NotFound)));

        let result = service.get_wms_by_id(1, 2).await;
        assert!(matches!(result, Err(RepoError::Forbidden)));
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

        mock_repo
            .expect_get_wms_groups()
            .with(eq(1))
            .returning(|_| {
                Ok(vec![WmsGroup {
                    id: 1,
                    name: "test".to_string(),
                    wms: None,
                    sub_groups: None,
                }])
            });

        let service = WmsService::new(mock_repo);

        let wms_group = service.get_wms_groups(1).await.unwrap();
        assert_eq!(wms_group.len(), 1);
        assert_eq!(wms_group[0].name, "test");
    }
}
