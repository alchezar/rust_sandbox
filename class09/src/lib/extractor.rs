pub mod example_1 {
	use actix_web::{Responder, web};

	struct MyInfo {
		id: u32,
		username: String,
	}

	async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
		let path = path.into_inner();
		format!("{} {} {} {}", path.0, path.1, json.id, json.username)
	}
}

// [path](https://actix.rs/docs/extractors#path)
pub mod example_2 {
	use actix_web::{App, HttpRequest, HttpServer, Responder, get, web};

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		HttpServer::new(|| {
			App::new()
				.service(index_1)
				.service(index_2)
		})
		.bind("localhost:8080")?
		.run()
		.await
	}

	// Extract into tuple.
	#[get("/users/{user_id}/{friend}")]
	async fn index_1(path: web::Path<(u32, String)>) -> std::io::Result<String> {
		let (user_id, friend) = path.into_inner();
		Ok(format!("Welcome {}, user_id {}", friend, user_id))
	}

	// Extract into struct.
	#[derive(serde::Deserialize)]
	struct Info {
		user_id: u32,
		friend: String,
	}

	#[get("/users/{user_id}/{friend}")]
	async fn index_2(info: web::Path<Info>) -> std::io::Result<String> {
		Ok(format!("Welcome {}, user_id {}", info.friend, info.user_id))
	}

	// Extract into query.
	#[get("/users/{user_id}/{friend}")]
	async fn index_3(req: HttpRequest) -> std::io::Result<String> {
		let name = req
			.match_info()
			.get("friend")
			.unwrap()
			.parse::<String>()
			.unwrap();
		let user_id = req
			.match_info()
			.query("user_id")
			.parse::<i32>()
			.unwrap();

		Ok(format!("Welcome {}, user_id {}", name, user_id))
	}
}

/// [Query](https://actix.rs/docs/extractors#query)
///
/// Getting the query from the URL after `?`.
/// For requests like:
/// ```curl
/// GET http://localhost:8080/?username=john
/// ```
pub mod example_3 {
	use actix_web::{get, web};

	#[derive(serde::Deserialize)]
	struct Info1 {
		username: String,
	}

	#[get("/")]
	async fn index_1(info: web::Query<Info1>) -> String {
		format!("Welcome {}!", info.username)
	}

	#[derive(serde::Deserialize)]
	struct Info2 {
		username: Option<String>,
	}

	#[get("/")]
	async fn index_2(info: web::Query<Info2>) -> String {
		match &info.username {
			Some(name) => format!("Welcome {}!", name),
			None => "Welcome anonymous!".into(),
		}
	}
}

// [JSON](https://actix.rs/docs/extractors#json)
pub mod example_4 {
	use actix_web::{App, HttpResponse, HttpServer, Responder, error, post, web};

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		HttpServer::new(|| {
			let json_config = web::JsonConfig::default()
				.limit(4096)
				.error_handler(|err, _req| {
					error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
				});

			App::new().service(
				web::resource("/submit")
					.app_data(json_config)
					.route(web::post().to(index)),
			)
		})
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
	}

	#[derive(serde::Deserialize)]
	struct Info {
		username: String,
	}

	async fn index(info: web::Json<Info>) -> std::io::Result<String> {
		Ok(format!("Welcome {}", info.username))
	}
}

/// [URL-Encoded Forms](https://actix.rs/docs/extractors#url-encoded-forms)
///
/// Getting the data from the body of the HTTP request.
/// For requests like:
/// ```curl
/// POST / HTTP/1.1
/// Content-Type: application/x-www-form-urlencoded
/// Content-Length: 13
///
/// username=john
/// ```
pub mod example_5 {
	use actix_web::{post, web};

	#[derive(serde::Deserialize)]
	struct FormData {
		username: String,
	}

	#[post("/")]
	async fn index(form: web::Form<FormData>) -> std::io::Result<String> {
		Ok(format!("Welcome {}", form.username))
	}
}

/// [Application State Extractor](https://actix.rs/docs/extractors#application-state-extractor)
///
pub mod example_6 {
	use actix_web::{App, HttpServer, Responder, web};
	use std::cell::Cell;
	use std::sync::Arc;
	use std::sync::atomic::{AtomicUsize, Ordering};
	use tokio::sync::RwLock;

	#[derive(Clone)]
	struct AppState {
		// count: Arc<RwLock<usize>>,
		global_count: Arc<AtomicUsize>,
	}

	async fn show_count(data: web::Data<AppState>) -> impl Responder {
		// format!("count: {}", data.count.read().await)
		format!(
			"count: {}",
			data.global_count
				.load(Ordering::Acquire)
		)
	}

	async fn add_one(data: web::Data<AppState>) -> impl Responder {
		// *data.count.write().await += 1;
		// format!("count: {}", data.count.read().await)
		data.global_count
			.fetch_add(1, Ordering::AcqRel);
		format!(
			"count: {}",
			data.global_count
				.load(Ordering::Acquire)
		)
	}

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		let data = AppState {
			global_count: Arc::new(AtomicUsize::new(0)),
		};
		HttpServer::new(move || {
			App::new()
				.app_data(web::Data::new(data.clone()))
				.route("/", web::to(show_count))
				.route("/add", web::to(add_one))
		})
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
	}
}
