// IKinder

use axum::Json;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing;

#[tokio::main]
pub async fn main() {
    crate::show_name(file!());

    let app = axum::Router::new()
        .route("/", routing::get(say_hello))
        .route("/text", routing::get(say_hello_text))
        .route("/page", routing::get(say_hello_page))
        .route("/json", routing::get(say_hello_json))
        .route("/post", routing::post(hello_post));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn say_hello() -> Html<&'static str> {
    "<h1>Hello World!</h1>".into()
}

async fn say_hello_text() -> Html<&'static str> {
    // "<h1>Hello World!</h1>".into()
    const HTML: &str = include_str!("./web_service/hello.html");
    Html(HTML)
}

async fn say_hello_page() -> Html<String> {
    let path = std::path::Path::new("./src/theme/web_service/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    content.into()
}

#[derive(serde::Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_json() -> Json<HelloJson> {
    let message = HelloJson {
        message: "Hi from JSON".to_owned(),
    };
    Json(message)
}

async fn hello_post() -> Html<String> {
    Html("Hello from post".into())
}
