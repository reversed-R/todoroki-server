use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

use todoroki_domain::value_objects::error::ErrorCode;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    code: ErrorResponseCode,
    message: String,
}

#[derive(Debug, Serialize, ToSchema)]
enum ErrorResponseCode {
    #[serde(rename = "todo/not-found")]
    TodoNotFound,
    #[serde(rename = "doit/not-found")]
    DoitNotFound,
    #[serde(rename = "label/not-found")]
    LabelNotFound,
    #[serde(rename = "permission/denied")]
    PermissionDenied,
    #[serde(rename = "todo/repository-internal-error")]
    TodoRepositoryInternalError,
    #[serde(rename = "doit/repository-internal-error")]
    DoitRepositoryInternalError,
    #[serde(rename = "label/repository-internal-error")]
    LabelRepositoryInternalError,
    #[serde(rename = "user/repository-internal-error")]
    UserRepositoryInternalError,
    #[serde(rename = "user-auth/token-verification-error")]
    UserAuthTokenVerificationError,
    #[serde(rename = "user-auth/not-verified")]
    UserNotVerified,
    #[serde(rename = "user/not-found")]
    UserNotFound,
    #[serde(rename = "datetime/invalid-format")]
    InvalidDateTimeFormat,
    #[serde(rename = "uuid/invalid-format")]
    InvalidUuidFormat,
    #[serde(rename = "color/invalid-format")]
    InvalidColorFormat,
}

impl From<ErrorCode> for ErrorResponse {
    fn from(value: ErrorCode) -> Self {
        Self {
            code: ErrorResponseCode::from(&value),
            message: value.to_string(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status_code = match self.code {
            ErrorResponseCode::TodoNotFound => StatusCode::NOT_FOUND,
            ErrorResponseCode::DoitNotFound => StatusCode::NOT_FOUND,
            ErrorResponseCode::LabelNotFound => StatusCode::NOT_FOUND,
            ErrorResponseCode::PermissionDenied => StatusCode::FORBIDDEN,
            ErrorResponseCode::TodoRepositoryInternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponseCode::DoitRepositoryInternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponseCode::LabelRepositoryInternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponseCode::UserRepositoryInternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponseCode::UserAuthTokenVerificationError => StatusCode::UNAUTHORIZED,
            ErrorResponseCode::UserNotVerified => StatusCode::UNAUTHORIZED,
            ErrorResponseCode::UserNotFound => StatusCode::NOT_FOUND,
            ErrorResponseCode::InvalidDateTimeFormat => StatusCode::BAD_REQUEST,
            ErrorResponseCode::InvalidUuidFormat => StatusCode::BAD_REQUEST,
            ErrorResponseCode::InvalidColorFormat => StatusCode::BAD_REQUEST,
        };

        (status_code, Json(self)).into_response()
    }
}

impl From<&ErrorCode> for ErrorResponseCode {
    fn from(value: &ErrorCode) -> Self {
        match value {
            ErrorCode::TodoNotFound(_) => Self::TodoNotFound,
            ErrorCode::DoitNotFound(_) => Self::DoitNotFound,
            ErrorCode::LabelNotFound(_) => Self::LabelNotFound,
            ErrorCode::PermissionDenied(_) => Self::PermissionDenied,
            ErrorCode::TodoRepositoryInternalError(_) => Self::TodoRepositoryInternalError,
            ErrorCode::DoitRepositoryInternalError(_) => Self::DoitRepositoryInternalError,
            ErrorCode::LabelRepositoryInternalError(_) => Self::LabelRepositoryInternalError,
            ErrorCode::UserRepositoryInternalError(_) => Self::UserRepositoryInternalError,
            ErrorCode::UserAuthTokenVerificationError(_) => Self::UserAuthTokenVerificationError,
            ErrorCode::UserNotVerified => Self::UserNotVerified,
            ErrorCode::UserNotFound(_) => Self::UserNotFound,
            ErrorCode::InvalidDateTimeFormat(_) => Self::InvalidDateTimeFormat,
            ErrorCode::InvalidUuidFormat(_) => Self::InvalidUuidFormat,
            ErrorCode::InvalidColorFormat(_) => Self::InvalidColorFormat,
        }
    }
}
