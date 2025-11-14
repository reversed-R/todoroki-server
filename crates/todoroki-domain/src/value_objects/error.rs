use std::fmt::Display;

// use getset::Getters;
use thiserror::Error;

use crate::{
    entities::{todo::TodoId, user::UserId},
    repositories::{todo::TodoRepositoryError, user::UserRepositoryError},
    value_objects::permission::Permission,
};

#[derive(Debug, Clone, Error)]
pub enum ErrorCode {
    TodoNotFound(TodoId),
    PermissionDenied(Permission),
    #[error(transparent)]
    TodoRepositoryInternalError(#[from] TodoRepositoryError),
    #[error(transparent)]
    UserRepositoryInternalError(#[from] UserRepositoryError),
    InvalidDateTimeFormat(String),
    InvalidUuidFormat(String),
    UserAuthTokenVerificationError(String),
    UserNotVerified,
    UserNotFound(UserId),
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TodoNotFound(id) => write!(f, "todo/not-found; id={}", id.clone().value()),
            Self::PermissionDenied(perm) => write!(f, "permission/denied; permission={perm}"),
            Self::TodoRepositoryInternalError(e) => {
                write!(f, "todo/repository-internal-error; error={e}")
            }
            Self::InvalidDateTimeFormat(s) => write!(f, "datetime/invalid-format; error={s}"),
            Self::InvalidUuidFormat(s) => write!(f, "uuid/invalid-format; string={s}"),
            Self::UserRepositoryInternalError(e) => {
                write!(f, "user/repository-internal-error; error={e}")
            }
            Self::UserAuthTokenVerificationError(s) => {
                write!(f, "user-auth/token-verification-failed; error={s}")
            }
            Self::UserNotVerified => {
                write!(f, "user-auth/not-verified")
            }
            Self::UserNotFound(id) => {
                write!(f, "user/not-found; id={}", id.clone().value())
            }
        }
    }
}
