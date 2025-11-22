use std::sync::Arc;

use todoroki_presentation::{config::Config, modules, routes};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("application initializing...");

    let config = Config::load().unwrap();
    let modules = modules::default(config).await.unwrap();

    let app = routes::router(Arc::new(modules));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("application starts!");
    axum::serve(listener, app).await.unwrap();
}
