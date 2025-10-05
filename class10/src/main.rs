use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

// cargo watch -q -c -w src\ -x run

#[tokio::main]
pub async fn main() {
	class10::show_name(file!());

	let routes = Router::new()
		.merge(class10::web::hello::routes())
		.merge(class10::web::routes_login::routes())
		.fallback_service(routes_static());

	let addr = SocketAddr::from(([127, 0, 0, 1], 6666));
	let listener = tokio::net::TcpListener::bind(addr)
		.await
		.unwrap();
	axum::serve(listener, routes)
		.await
		.unwrap();
}

fn routes_static() -> axum::routing::MethodRouter {
	// curl http://localhost:6666/src/main.rs
	axum::routing::get_service(ServeDir::new("./"))
}
