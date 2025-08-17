//! * `scope` - groups routes under a common path prefix.
//!
//! * `resource` - one URL, different HTTP methods.
//! Binds handler functions to different HTTP methods (GET, POST, PUT ...) for a
//! single URL path. (e.g. `GET /api/user` or `PUT api/user`)
//!
//! * `guar` - condition filters that determine which route/handler should be
//! matched based on request properties (host, headers ...).

use actix_web::{App, HttpServer, Responder, web};

/// [Application](https://actix.rs/docs/application)
pub mod example_1 {
	use super::*;

	#[actix_web::main]
	pub async fn main() -> std::io::Result<()> {
		HttpServer::new(|| {
			App::new().service(
				// Prefixes all resources and routers attached to it.
				// So this handles request for `GET /app/index.html`.
				web::scope("/app").route("/index.html", web::get().to(index)),
			)
		})
		.bind("127.0.0.1:8080")?
		.run()
		.await
	}

	pub async fn index() -> impl Responder {
		"Hello actix"
	}
}

/// [State](https://actix.rs/docs/application/#state)
pub mod example_2 {
	use super::*;

	#[actix_web::main]
	pub async fn main() -> std::io::Result<()> {
		HttpServer::new(|| {
			App::new()
				.app_data(web::Data::new(AppState { app_name: "Actix Web".into() }))
				.service(index)
		})
		.bind("127.0.0.1:8080")?
		.run()
		.await
	}

	struct AppState {
		app_name: String,
	}

	#[actix_web::get("/")]
	async fn index(data: web::Data<AppState>) -> impl Responder {
		let app_name = &data.app_name;
		format!("Hello {}!", app_name)
	}
}

/// [Shared Mutable State](https://actix.rs/docs/application/#shared-mutable-state)
pub mod example_3 {
	use crate::lib::application::example_1;
	use actix_web::{App, HttpServer, web};
	use std::sync::Mutex;

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		let counter = web::Data::new(AppStateWithCounter { counter: Mutex::new(0) });

		HttpServer::new(move || {
			App::new()
				.app_data(counter.clone())
				.route("/", web::get().to(index))
		})
		.bind("localhost:8080")?
		.run()
		.await
	}

	struct AppStateWithCounter {
		// Mutex is necessary to mutate safely across threads.
		counter: Mutex<i32>,
	}

	async fn index(data: web::Data<AppStateWithCounter>) -> String {
		let mut counter = data.counter.lock().unwrap();
		*counter += 1;

		format!("Request number: {}", counter)
	}
}

/// [Application guards and virtual hosting](https://actix.rs/docs/application/#application-guards-and-virtual-hosting)
pub mod example_4 {
	use actix_web::{App, HttpResponse, HttpServer, guard, web};

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		HttpServer::new(|| {
			App::new()
				.service(
					web::scope("/")
						.guard(guard::Host("www.rust-lang.org"))
						.route("", web::to(|| async { HttpResponse::Ok().body("www") })),
				)
				.service(
					web::scope("/")
						.guard(guard::Host("users.rust-lang.org"))
						.route("", web::to(|| async { HttpResponse::Ok().body("users") })),
				)
				.service(
					web::scope("/")
						.guard(guard::Host("admin.rust-lang.org").scheme("https"))
						.default_service(web::to(|| async { HttpResponse::Ok().body("admin") })),
				)
				.route("/", web::to(HttpResponse::Ok))
				.route(
					"/test",
					web::get()
						.guard(guard::Header("content-type", "text/html"))
						.to(|| async { HttpResponse::Ok().body("test") }),
				)
		})
		.bind("localhost:8080")?
		.run()
		.await
	}
}

/// [Configure](https://actix.rs/docs/application/#configure)
pub mod example_5 {
	use actix_web::{App, HttpResponse, HttpServer, web};

	// This function could be located in a different module.
	fn scoped_config(cfg: &mut web::ServiceConfig) {
		cfg.service(
			web::resource("/test")
				.route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
				.route(web::head().to(HttpResponse::MethodNotAllowed)),
		);
	}

	// This function could be located in a different module.
	fn config(cfg: &mut web::ServiceConfig) {
		cfg.service(
			web::resource("/app")
				.route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
				.route(web::head().to(HttpResponse::MethodNotAllowed)),
		);
	}

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		HttpServer::new(|| {
			App::new()
				.configure(config)
				.service(web::scope("/api").configure(scoped_config))
				.route("/", web::get().to(|| async { HttpResponse::Ok().body("/") }))
		})
		.bind("localhost:8080")?
		.run()
		.await
	}
}
