use todoroki_domain::value_objects::error::ErrorCode;

use crate::todo::TodoUseCaseError;

impl From<TodoUseCaseError> for ErrorCode {
    fn from(value: TodoUseCaseError) -> Self {
        match value {
            TodoUseCaseError::TodoRepositoryError(e) => Self::TodoRepositoryInternalError(e),
        }
    }
}
