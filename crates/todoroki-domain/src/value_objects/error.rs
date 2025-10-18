use std::fmt::Display;

// use getset::Getters;
use thiserror::Error;

use crate::{entities::todo::TodoId, repositories::todo::TodoRepositoryError};

#[derive(Debug, Clone, Error)]
pub enum ErrorCode {
    TodoNotFound(TodoId),
    PermissionDenied,
    #[error(transparent)]
    TodoRepositoryInternalError(#[from] TodoRepositoryError),
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TodoNotFound(id) => write!(f, "todo/not-found; id={}", id.clone().value()),
            Self::PermissionDenied => write!(f, "permission/denied"),
            Self::TodoRepositoryInternalError(e) => {
                write!(f, "todo/repository-internal-error; error={e}")
            }
        }
    }
}
