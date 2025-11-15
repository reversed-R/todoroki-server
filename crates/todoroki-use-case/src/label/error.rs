use todoroki_domain::value_objects::error::ErrorCode;

use crate::label::LabelUseCaseError;

impl From<LabelUseCaseError> for ErrorCode {
    fn from(value: LabelUseCaseError) -> Self {
        match value {
            LabelUseCaseError::LabelRepositoryError(e) => Self::LabelRepositoryInternalError(e),
        }
    }
}
