use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use std::sync::Arc;
use todoroki_domain::entities::todo::TodoId;

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
    path = "/todos",
    operation_id = "getTodos",
    tag = "todo",
    responses(
        (status = 200, description = "OK", body = Vec<responses::todo::TodoResponse>),
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
    let res = modules.todo_use_case().list(&ctx).await;

    match res {
        Ok(todos) => Ok(Json(
            todos
                .iter()
                .map(responses::todo::TodoResponse::from)
                .collect::<Vec<responses::todo::TodoResponse>>(),
        )),
        Err(e) => Err(e.into()),
    }
}

#[utoipa::path(
    post,
    path = "/todos",
    operation_id = "postTodo",
    tag = "todo",
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
    Json(raw_todo): Json<requests::todo::TodoRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let todo = raw_todo.try_into()?;

    let res = modules.todo_use_case().create(todo, &ctx).await;

    match res {
        Ok(id) => Ok(SuccessResponse::new(format!(
            "todo/created; id={}",
            id.value().as_hyphenated()
        ))),
        Err(e) => Err(e.into()),
    }
}

#[utoipa::path(
    patch,
    path = "/todos/{todo_id}",
    operation_id = "patchTodoById",
    tag = "todo",
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
    Json(raw_cmd): Json<requests::todo::TodoUpdateCommand>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let id = TodoId::try_from(raw_id)?;
    let cmd = raw_cmd.try_into_with_id(id)?;

    let res = modules.todo_use_case().update(cmd, &ctx).await;

    match res {
        Ok(id) => Ok(SuccessResponse::new(format!("todo/updated",))),
        Err(e) => Err(e.into()),
    }
}
