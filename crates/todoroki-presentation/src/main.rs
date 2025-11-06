mod config;
mod middlewares;
mod models;
mod modules;
mod routes;

use std::sync::Arc;

use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load().unwrap();
    let modules = modules::default(config).await.unwrap();

    let app = routes::router(Arc::new(modules));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
