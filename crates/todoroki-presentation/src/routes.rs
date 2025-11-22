pub mod todo;
pub mod user;
pub mod health;
pub mod label;
pub mod doit;

use crate::{middlewares, modules::Modules};
use todoroki_infrastructure::shared::DefaultRepositories;

use axum::{http::{header, Method}, routing::{get, patch, post}, Router};
use tracing::Level;
use std::sync::Arc;
use tower_http::{cors::{Any, CorsLayer}, trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer}};
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
    
    // doit の作成/更新操作は常に認証を要する
    let doit_auth_routes = Router::new()
        .route("/", post(doit::handle_post))
        .route("/{doit_id}", patch(doit::handle_patch))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::clone(&modules),
            middlewares::auth::jwt_auth,
        ));

    // doit の取得は必ずしも認証しなくても良い
    let doit_opt_auth_routes = Router::new()
        .route("/", get(doit::handle_get))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::clone(&modules),
            middlewares::auth::optional_jwt_auth,
        ));
    
    let doit_routes = Router::new()
        .nest("/doits", doit_opt_auth_routes)
        .nest("/doits", doit_auth_routes);
    
    // label の作成/更新操作は常に認証を要する
    let label_auth_routes = Router::new()
        .route("/", post(label::handle_post))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::clone(&modules),
            middlewares::auth::jwt_auth,
        ));

    // label の取得は必ずしも認証しなくても良い
    let label_opt_auth_routes = Router::new()
        .route("/", get(label::handle_get))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::clone(&modules),
            middlewares::auth::optional_jwt_auth,
        ));
    
    let label_routes = Router::new()
        .nest("/labels", label_opt_auth_routes)
        .nest("/labels", label_auth_routes);
    
    // user の作成操作は常に認証を要する
    let user_auth_routes = Router::new()
        .route("/", post(user::handle_post))
        .route("/me", get(user::handle_get_me))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::clone(&modules),
            middlewares::auth::jwt_auth,
        ));
    
    let user_routes = Router::new()
        .nest("/users", user_auth_routes);
    
    Router::new()
        .route("/health", get(health::handle_health))
        .merge(todo_routes)
        .merge(doit_routes)
        .merge(label_routes)
        .merge(user_routes)
        .with_state(modules)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            CorsLayer::new()
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::PATCH, Method::POST])
                .allow_origin(Any),
        )
}

use crate::routes;
#[derive(OpenApi)]
#[openapi(
    info(title = "Todoroki API", license(name = "MIT", identifier = "MIT")),
    tags(
        (name = "health", description = "APIの死活チェック"),
        (name = "todo", description = "Todo関連の操作"),
        (name = "doit", description = "Do it! 関連の操作"),
        (name = "label", description = "ラベル関連の操作"),
        (name = "user", description = "ユーザー関連の操作"),
    ), 
    paths(
        routes::health::handle_health,
        routes::todo::handle_get,
        routes::todo::handle_post,
        routes::todo::handle_patch,
        routes::doit::handle_get,
        routes::doit::handle_post,
        routes::doit::handle_patch,
        routes::label::handle_get,
        routes::label::handle_post,
        routes::user::handle_post,
        routes::user::handle_get_me,
    )
)]
pub struct ApiDocs;
