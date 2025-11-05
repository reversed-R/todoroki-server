pub mod error;
pub mod operations;

use std::sync::Arc;
use thiserror::Error;

use todoroki_domain::repositories::{user::UserRepositoryError, Repositories};

pub struct UserUseCase<R: Repositories> {
    repositories: Arc<R>,
}

#[derive(Debug, Error)]
pub enum UserUseCaseError {
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
}

impl<R: Repositories> UserUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
