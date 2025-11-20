use std::fmt::Display;

// use getset::Getters;
use thiserror::Error;

use crate::{
    entities::{
        doit::DoitId,
        label::LabelId,
        todo::TodoId,
        user::{UserEmail, UserId},
    },
    repositories::{
        doit::DoitRepositoryError, label::LabelRepositoryError, todo::TodoRepositoryError,
        user::UserRepositoryError,
    },
    value_objects::permission::Permission,
};

#[derive(Debug, Clone, Error)]
pub enum ErrorCode {
    TodoNotFound(TodoId),
    DoitNotFound(DoitId),
    LabelNotFound(LabelId),
    PermissionDenied(Box<Permission>),
    #[error(transparent)]
    TodoRepositoryInternalError(#[from] TodoRepositoryError),
    #[error(transparent)]
    DoitRepositoryInternalError(#[from] DoitRepositoryError),
    #[error(transparent)]
    LabelRepositoryInternalError(#[from] LabelRepositoryError),
    #[error(transparent)]
    UserRepositoryInternalError(#[from] UserRepositoryError),
    UserAuthTokenVerificationError(String),
    UserNotVerified,
    UserNotFound(UserId),
    UserAlreadyExistsForEmail(UserEmail),
    InvalidDateTimeFormat(String),
    InvalidUuidFormat(String),
    InvalidColorFormat(String),
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TodoNotFound(id) => write!(f, "todo/not-found; id={}", id.clone().value()),
            Self::DoitNotFound(id) => write!(f, "doit/not-found; id={}", id.clone().value()),
            Self::LabelNotFound(id) => write!(f, "label/not-found; id={}", id.clone().value()),
            Self::PermissionDenied(perm) => write!(f, "permission/denied; permission={perm}"),
            Self::TodoRepositoryInternalError(e) => {
                write!(f, "todo/repository-internal-error; error={e}")
            }
            Self::DoitRepositoryInternalError(e) => {
                write!(f, "doit/repository-internal-error; error={e}")
            }
            Self::LabelRepositoryInternalError(e) => {
                write!(f, "label/repository-internal-error; error={e}")
            }
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
            Self::UserAlreadyExistsForEmail(email) => {
                write!(f, "user/already-exists; email={}", email.clone().value())
            }
            Self::InvalidDateTimeFormat(s) => write!(f, "datetime/invalid-format; error={s}"),
            Self::InvalidUuidFormat(s) => write!(f, "uuid/invalid-format; string={s}"),
            Self::InvalidColorFormat(s) => write!(f, "color/invalid-format; string={s}"),
        }
    }
}
