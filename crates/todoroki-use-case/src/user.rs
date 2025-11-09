pub mod error;
pub mod operations;

use std::sync::Arc;
use thiserror::Error;

use todoroki_domain::repositories::{user::UserRepositoryError, Repositories};

pub struct UserUseCase<R: Repositories> {
    firebase_project_id: String,
    repositories: Arc<R>,
}

#[derive(Debug, Error)]
pub enum UserUseCaseError {
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    UserAuthTokenVerificationError(String),
    UserAuthTokenKeyNotFound(String),
}

impl<R: Repositories> UserUseCase<R> {
    pub fn new(repositories: Arc<R>, firebase_project_id: String) -> Self {
        Self {
            firebase_project_id,
            repositories,
        }
    }
}
