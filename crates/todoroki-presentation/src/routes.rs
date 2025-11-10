pub mod todo;
pub mod user;
pub mod health;

use crate::{middlewares, modules::Modules};
use todoroki_infrastructure::shared::DefaultRepositories;

use axum::{routing::{get, patch, post}, Router};
use std::sync::Arc;
use utoipa::OpenApi;

pub fn router(modules: Arc<Modules<DefaultRepositories>>) -> Router {
    // todo の作成/更新操作は常に認証を要する
    let todo_auth_routes = Router::new()
        .route("/", post(todo::handle_post))
        .route("/{todo_id}", patch(todo::handle_patch))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::clone(&modules),
            middlewares::auth::jwt_auth,
        ));

    // todo の取得は必ずしも認証しなくても良い
    let todo_opt_auth_routes = Router::new()
        .route("/", get(todo::handle_get))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::clone(&modules),
            middlewares::auth::optional_jwt_auth,
        ));
    
    let todo_routes = Router::new()
        .nest("/todos", todo_opt_auth_routes)
        .nest("/todos", todo_auth_routes);
    
    // user の作成操作は常に認証を要する
    let user_auth_routes = Router::new()
        .route("/", post(user::handle_post))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::clone(&modules),
            middlewares::auth::jwt_auth,
        ));
    
    let user_routes = Router::new()
        .nest("/users", user_auth_routes);
    
    Router::new()
        .route("/health", get(health::handle_health))
        .merge(todo_routes)
        .merge(user_routes)
        .with_state(modules)
}

use crate::routes;
#[derive(OpenApi)]
#[openapi(
    info(title = "Todoroki API", license(name = "MIT", identifier = "MIT")),
    tags(
        (name = "health", description = "APIの死活チェック"),
        (name = "todo", description = "Todo関連の操作"),
        (name = "user", description = "ユーザー関連の操作"),
    ), 
    paths(
        routes::health::handle_health,
        routes::todo::handle_get,
        routes::todo::handle_post,
        routes::todo::handle_patch,
        routes::user::handle_post,
    )
)]
pub struct ApiDocs;
