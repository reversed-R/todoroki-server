pub mod error;
pub mod operations;

use std::sync::Arc;
use thiserror::Error;

use todoroki_domain::repositories::{label::LabelRepositoryError, Repositories};

pub struct LabelUseCase<R: Repositories> {
    repositories: Arc<R>,
}

#[derive(Debug, Error)]
pub enum LabelUseCaseError {
    #[error(transparent)]
    LabelRepositoryError(#[from] LabelRepositoryError),
}

impl<R: Repositories> LabelUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
