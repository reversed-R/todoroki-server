use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

use crate::{
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
        (status = 200, description = "OK", body = Vec<responses::todo::Todo>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(()),
)]
pub async fn handle_get(
    State(modules): State<Arc<Modules<DefaultRepositories>>>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let res = modules.todo_use_case().list().await;

    match res {
        Ok(todos) => Ok(Json(
            todos
                .iter()
                .map(responses::todo::Todo::from)
                .collect::<Vec<responses::todo::Todo>>(),
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
    security(()),
)]
pub async fn handle_post(
    State(modules): State<Arc<Modules<DefaultRepositories>>>,
    Json(raw_todo): Json<requests::todo::Todo>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let todo = raw_todo.try_into()?;

    let res = modules.todo_use_case().create(todo).await;

    match res {
        Ok(id) => Ok(SuccessResponse::new(format!(
            "todo/created; id={}",
            id.value().as_hyphenated()
        ))),
        Err(e) => Err(e.into()),
    }
}
