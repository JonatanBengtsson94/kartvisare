#[derive(Debug)]
pub enum RepoError {
    NotFound,
    Forbidden,
    DatabaseError(sqlx::Error),
}
