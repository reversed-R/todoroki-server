use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use std::sync::Arc;
use todoroki_domain::{
    entities::{client::Client, doit::DoitId},
    value_objects::error::ErrorCode,
};
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
    path = "/doits",
    operation_id = "getDoits",
    tag = "doit",
    responses(
        (status = 200, description = "OK", body = Vec<responses::doit::DoitResponse>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(("jwt_token" = []), ("nothing" = [])),
)]
pub async fn handle_get(
    State(modules): State<Arc<Modules<DefaultRepositories>>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let res = modules.doit_use_case().list(&ctx).await;

    match res {
        Ok(doits) => Ok(Json(
            doits
                .into_iter()
                .map(responses::doit::DoitResponse::from)
                .collect::<Vec<responses::doit::DoitResponse>>(),
        )),
        Err(e) => Err(e.into()),
    }
}

#[utoipa::path(
    post,
    path = "/doits",
    operation_id = "postDoit",
    tag = "doit",
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
    Json(raw_doit): Json<requests::doit::DoitRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let labels = modules
        .label_use_case()
        .list(&ctx)
        .await
        .map_err(ErrorCode::from)?;

    let user_id = if let Client::User(u) = ctx.client().client() {
        u.id().clone()
    } else {
        return Err(ErrorResponse::from(ErrorCode::UserNotVerified));
    };

    let doit = raw_doit.try_into_with_labels_and_created_by(labels, user_id)?;

    let res = modules.doit_use_case().create(doit, &ctx).await;

    match res {
        Ok(id) => Ok(SuccessResponse::new(format!(
            "doit/created; id={}",
            id.value().as_hyphenated()
        ))),
        Err(e) => Err(e.into()),
    }
}

#[utoipa::path(
    patch,
    path = "/doits/{doit_id}",
    operation_id = "patchDoitById",
    tag = "doit",
    responses(
        (status = 201, description = "Updated", body = SuccessResponse),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_patch(
    Path(raw_id): Path<String>,
    State(modules): State<Arc<Modules<DefaultRepositories>>>,
    Extension(ctx): Extension<Context>,
    Json(raw_cmd): Json<requests::doit::DoitUpdateCommand>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let id = DoitId::try_from(raw_id)?;
    let cmd = raw_cmd.try_into_with_id(id)?;

    let res = modules.doit_use_case().update(cmd, &ctx).await;

    match res {
        Ok(()) => Ok(SuccessResponse::new(format!("doit/updated"))),
        Err(e) => Err(e.into()),
    }
}
