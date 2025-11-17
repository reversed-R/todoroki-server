pub mod dto;
pub mod error;
pub mod operations;

use std::sync::Arc;
use thiserror::Error;

use todoroki_domain::repositories::{doit::DoitRepositoryError, Repositories};

pub struct DoitUseCase<R: Repositories> {
    repositories: Arc<R>,
}

#[derive(Debug, Error)]
pub enum DoitUseCaseError {
    #[error(transparent)]
    DoitRepositoryError(#[from] DoitRepositoryError),
}

impl<R: Repositories> DoitUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
