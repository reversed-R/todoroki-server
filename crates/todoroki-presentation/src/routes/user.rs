use axum::{extract::State, response::IntoResponse, Extension, Json};
use std::sync::Arc;
use todoroki_domain::{entities::client::Client, value_objects::error::ErrorCode};
use todoroki_use_case::shared::ContextProvider;

use crate::{
    context::Context,
    models::{
        requests,
        responses::{self, error::ErrorResponse, success::SuccessResponse},
    },
    modules::Modules,
};
use todoroki_infrastructure::shared::DefaultRepositories;

#[utoipa::path(
    get,
    path = "/users/me",
    operation_id = "getUserOwn",
    tag = "user",
    responses(
        (status = 200, description = "OK", body = Vec<responses::user::UserResponse>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_get_me(
    State(modules): State<Arc<Modules<DefaultRepositories>>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, ErrorResponse> {
    match ctx.client() {
        Client::User(user) => Ok(Json(responses::user::UserResponse::from(user.to_owned()))),
        Client::Unregistered(_) => Err(ErrorCode::UserNotVerified.into()),
        Client::Unverified => Err(ErrorCode::UserNotVerified.into()),
    }
}

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
    Json(raw_user): Json<requests::user::UserRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let email = match ctx.client() {
        Client::User(_) => todo!(), // user already exists for the email
        Client::Unverified => return Err(ErrorResponse::from(ErrorCode::UserNotVerified)),
        Client::Unregistered(email) => email,
    };

    let user = raw_user.try_into_with_email(email.clone())?;

    let res = modules.user_use_case().create(user, &ctx).await;

    match res {
        Ok(id) => Ok(SuccessResponse::new(format!(
            "user/created; id={}",
            id.value().as_hyphenated()
        ))),
        Err(e) => Err(e.into()),
    }
}
