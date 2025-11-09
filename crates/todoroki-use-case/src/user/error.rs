use std::fmt::Display;

use todoroki_domain::value_objects::error::ErrorCode;

use crate::user::UserUseCaseError;

impl From<UserUseCaseError> for ErrorCode {
    fn from(value: UserUseCaseError) -> Self {
        match value {
            UserUseCaseError::UserRepositoryError(e) => Self::UserRepositoryInternalError(e),
            UserUseCaseError::UserAuthTokenVerificationError(e) => {
                Self::UserAuthTokenVerificationError(e)
            }
            UserUseCaseError::UserAuthTokenKeyNotFound(k) => {
                Self::UserAuthTokenVerificationError(k)
            }
        }
    }
}

impl Display for UserUseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserUseCaseError::UserRepositoryError(e) => {
                write!(f, "user-use-case/repository-internal-error; error={e}")
            }
            UserUseCaseError::UserAuthTokenVerificationError(e) => {
                write!(f, "user-use-case/auth-token-verification-failed; error={e}")
            }
            UserUseCaseError::UserAuthTokenKeyNotFound(k) => {
                write!(f, "user-use-case/auth-token-key-not-found; kid={k}")
            }
        }
    }
}
