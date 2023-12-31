use std::fmt::Display;

pub mod template;

#[derive(Debug)]
pub enum RepoError {
    Internal(String),
}

impl From<sqlx::Error> for RepoError {
    fn from(error: sqlx::Error) -> Self {
        RepoError::Internal(error.to_string())
    }
}

impl Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::Internal(error) => write!(f, "{}", error),
        }
    }
}
