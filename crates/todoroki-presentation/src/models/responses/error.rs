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
    #[serde(rename = "permission/denied")]
    PermissionDenied,
    #[serde(rename = "todo/repository-internal-error")]
    TodoRepositoryInternalError,
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
            ErrorResponseCode::PermissionDenied => StatusCode::FORBIDDEN,
            ErrorResponseCode::TodoRepositoryInternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(self)).into_response()
    }
}

impl From<&ErrorCode> for ErrorResponseCode {
    fn from(value: &ErrorCode) -> Self {
        match value {
            ErrorCode::TodoNotFound(_) => Self::TodoNotFound,
            ErrorCode::PermissionDenied => Self::PermissionDenied,
            ErrorCode::TodoRepositoryInternalError(_) => Self::TodoRepositoryInternalError,
        }
    }
}
