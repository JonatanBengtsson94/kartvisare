#[derive(Debug)]
pub enum RepoError {
    NotFound,
    Forbidden,
    DatabaseError(sqlx::Error),
    RedisError(redis::RedisError),
    SerializationError(serde_json::Error),
}
