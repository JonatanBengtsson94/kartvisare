use async_trait::async_trait;

#[async_trait]
pub trait IdpService: Send + Sync {
    async fn validate_token(&self, token: &str) -> Result<i32, IdpError>;
}

#[derive(Debug)]
pub enum IdpError {
    InvalidToken,
}

#[derive(Clone)]
pub struct MockIdpService;

impl MockIdpService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IdpService for MockIdpService {
    async fn validate_token(&self, token: &str) -> Result<i32, IdpError> {
        // Mock implementation - in production, this would validate against the actual IDP
        if token == "valid_token" {
            Ok(1)
        } else if token == "admin_token" {
            Ok(2)
        } else {
            Err(IdpError::InvalidToken)
        }
    }
}
