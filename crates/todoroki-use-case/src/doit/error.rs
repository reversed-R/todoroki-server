use todoroki_domain::value_objects::error::ErrorCode;

use crate::doit::DoitUseCaseError;

impl From<DoitUseCaseError> for ErrorCode {
    fn from(value: DoitUseCaseError) -> Self {
        match value {
            DoitUseCaseError::DoitRepositoryError(e) => Self::DoitRepositoryInternalError(e),
            DoitUseCaseError::DoitNotFound(id) => Self::DoitNotFound(id),
        }
    }
}
