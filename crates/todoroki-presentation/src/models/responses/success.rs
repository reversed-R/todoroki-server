use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SuccessResponse {
    message: String,
}

impl SuccessResponse {
    pub(crate) fn new(message: String) -> Self {
        Self { message }
    }
}

impl IntoResponse for SuccessResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
