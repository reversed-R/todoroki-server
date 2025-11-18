pub mod dto;
pub mod error;
pub mod operations;

use std::sync::Arc;
use thiserror::Error;

use todoroki_domain::repositories::{todo::TodoRepositoryError, Repositories};

pub struct TodoUseCase<R: Repositories> {
    repositories: Arc<R>,
}

#[derive(Debug, Error)]
pub enum TodoUseCaseError {
    #[error(transparent)]
    TodoRepositoryError(#[from] TodoRepositoryError),
}

impl<R: Repositories> TodoUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
