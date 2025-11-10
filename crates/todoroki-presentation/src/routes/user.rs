use axum::{extract::State, response::IntoResponse, Extension, Json};
use std::sync::Arc;
use todoroki_domain::value_objects::error::ErrorCode;
use todoroki_use_case::shared::ContextProvider;

use crate::{
    context::Context,
    models::{
        requests,
        responses::{error::ErrorResponse, success::SuccessResponse},
    },
    modules::Modules,
};
use todoroki_infrastructure::shared::DefaultRepositories;

#[utoipa::path(
    post,
    path = "/users",
    operation_id = "postUser",
    tag = "user",
    responses(
        (status = 201, description = "Created", body = SuccessResponse),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_post(
    State(modules): State<Arc<Modules<DefaultRepositories>>>,
    Extension(ctx): Extension<Context>,
    Json(raw_user): Json<requests::user::User>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let user = raw_user.try_into_with_email(
        ctx.user_email()
            .clone()
            .ok_or(ErrorResponse::from(ErrorCode::UserNotVerified))?,
    )?;

    let res = modules.user_use_case().create(user, &ctx).await;

    match res {
        Ok(id) => Ok(SuccessResponse::new(format!(
            "user/created; id={}",
            id.value().as_hyphenated()
        ))),
        Err(e) => Err(e.into()),
    }
}
