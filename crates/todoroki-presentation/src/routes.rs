pub mod todo;
pub mod health;

use crate::modules::Modules;
use todoroki_infrastructure::shared::DefaultRepositories;

use axum::{routing::{get, patch, post}, Router};
use std::sync::Arc;
use utoipa::OpenApi;

pub fn router(modules: Arc<Modules<DefaultRepositories>>) -> Router {
    Router::new()
        .route("/health", get(health::handle_health))
        .route("/todos", post(todo::handle_post))
        .route("/todos", get(todo::handle_get))
        .route("/todos/{todo_id}", patch(todo::handle_patch))
        .with_state(modules)
}

use crate::routes;
#[derive(OpenApi)]
#[openapi(
    info(title = "Todo Manager API", license(name = "MIT", identifier = "MIT")),
    tags(
        (name = "health", description = "APIの死活チェック"),
        (name = "user", description = "ユーザー関連の操作"),
    ), 
    paths(
        routes::health::handle_health,
    )
)]
pub struct _ApiDocs;
