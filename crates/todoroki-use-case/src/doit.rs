pub mod dto;
pub mod error;
pub mod operations;

use std::sync::Arc;
use thiserror::Error;

use todoroki_domain::{
    entities::doit::DoitId,
    repositories::{doit::DoitRepositoryError, Repositories},
};

pub struct DoitUseCase<R: Repositories> {
    repositories: Arc<R>,
}

#[derive(Debug, Error)]
pub enum DoitUseCaseError {
    #[error(transparent)]
    DoitRepositoryError(#[from] DoitRepositoryError),
    #[error("Doit Not Found: {0:?}")]
    DoitNotFound(DoitId),
}

impl<R: Repositories> DoitUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
