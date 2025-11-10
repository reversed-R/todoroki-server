use std::sync::Arc;

use todoroki_presentation::{config::Config, modules, routes};

#[tokio::main]
async fn main() {
    let config = Config::load().unwrap();
    let modules = modules::default(config).await.unwrap();

    let app = routes::router(Arc::new(modules));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
