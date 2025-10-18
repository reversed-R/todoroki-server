use crate::models::responses::{error::ErrorResponse, success::SuccessResponse};

#[utoipa::path(
    get,
    path = "/health",
    operation_id = "checkHealth",
    tag = "health",
    responses(
        (status = 200, description = "OK", body = SuccessResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(()),
)]
pub async fn handle_health() -> SuccessResponse {
    SuccessResponse::new("OK".to_string())
}
