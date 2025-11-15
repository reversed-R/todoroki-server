use axum::{extract::State, response::IntoResponse, Extension, Json};
use std::sync::Arc;

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
    path = "/labels",
    operation_id = "getLabels",
    tag = "label",
    responses(
        (status = 200, description = "OK", body = Vec<responses::label::LabelResponse>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(),
)]
pub async fn handle_get(
    State(modules): State<Arc<Modules<DefaultRepositories>>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let res = modules.label_use_case().list(&ctx).await;

    match res {
        Ok(labels) => Ok(Json(
            labels
                .iter()
                .map(responses::label::LabelResponse::from)
                .collect::<Vec<responses::label::LabelResponse>>(),
        )),
        Err(e) => Err(e.into()),
    }
}

#[utoipa::path(
    post,
    path = "/labels",
    operation_id = "postLabel",
    tag = "label",
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
    Json(raw_label): Json<requests::label::LabelRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let label = raw_label.try_into()?;

    let res = modules.label_use_case().create(label, &ctx).await;

    match res {
        Ok(id) => Ok(SuccessResponse::new(format!(
            "label/created; id={}",
            id.value().as_hyphenated()
        ))),
        Err(e) => Err(e.into()),
    }
}
