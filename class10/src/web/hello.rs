use axum::Router;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        // curl http://localhost:6666
        .route(
            "/hello",
            get(|| async { Html("Hello <strong>World!!!</strong>") }),
        )
        // curl http://localhost:6666/hello2?name=John
        .route("/hello2", get(handler_query))
        // curl http://localhost:6666/hello3/John
        .route("/hello3/{name}", get(handler_path))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_query(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!!!");

    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
