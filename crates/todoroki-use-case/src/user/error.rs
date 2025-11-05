use todoroki_domain::value_objects::error::ErrorCode;

use crate::user::UserUseCaseError;

impl From<UserUseCaseError> for ErrorCode {
    fn from(value: UserUseCaseError) -> Self {
        match value {
            UserUseCaseError::UserRepositoryError(e) => Self::UserRepositoryInternalError(e),
        }
    }
}
