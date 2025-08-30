/// [errors](https: //actix.rs/docs/errors)
pub mod example_1 {
	use actix_web::HttpResponse;
	use actix_web::body::BoxBody;
	use actix_web::http::StatusCode;
	use derive_more::{Display, Error};

	pub trait ResponseErrorExample {
		fn error_response(&self) -> HttpResponse<BoxBody>;
		fn status_code(&self) -> StatusCode;
	}

	#[derive(Debug, Display, Error)]
	#[display("my error: {name}")]
	struct MyError {
		name: &'static str,
	}

	async fn index() -> Result<&'static str, MyError> {
		Err(MyError { name: "test" })
	}
}

pub mod example_2 {
	use actix_web::http::StatusCode;
	use actix_web::http::header::ContentType;
	use actix_web::{HttpResponse, ResponseError};
	use derive_more::{Display, Error};

	#[derive(Debug, Display, Error)]
	enum MyError {
		#[display("Internal error")]
		InternalError,
		#[display("Bad request")]
		BadClientData,
		#[display("timeout")]
		Timeout,
	}

	impl ResponseError for MyError {
		fn status_code(&self) -> StatusCode {
			match *self {
				MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
				MyError::BadClientData => StatusCode::BAD_REQUEST,
				MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
			}
		}
		fn error_response(&self) -> HttpResponse {
			HttpResponse::build(self.status_code())
				.insert_header(ContentType::html())
				.body(self.to_string())
		}
	}

	async fn index() -> Result<&'static str, MyError> {
		Err(MyError::BadClientData)
	}
}

//[Error helpers](https://actix.rs/docs/errors#error-helpers)
pub mod example_3 {
	use derive_more::{Display, Error};

	#[derive(Debug, Display, Error)]
	struct MyError {
		name: &'static str,
	}

	async fn index() -> actix_web::Result<String> {
		let result = Err(MyError { name: "test error" });
		result.map_err(|e| actix_web::error::ErrorBadRequest(e.name))
	}
}

// [Error handling](https://actix.rs/docs/errors#recommended-practices-in-error-handling)
pub mod example_4 {
	use actix_web::http::StatusCode;
	use actix_web::http::header::ContentType;
	use actix_web::{HttpResponse, ResponseError};
	use derive_more::{Display, Error};

	#[derive(Debug, Display, Error)]
	enum UserError {
		#[display("Validation error on field: {field}")]
		ValidationError { field: String },
	}

	impl ResponseError for UserError {
		fn status_code(&self) -> StatusCode {
			match *self {
				UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
			}
		}
		fn error_response(&self) -> HttpResponse {
			HttpResponse::build(self.status_code())
				.insert_header(ContentType::html())
				.body(self.to_string())
		}
	}
}

pub mod example_5 {
	use actix_web::http::StatusCode;
	use actix_web::http::header::ContentType;
	use actix_web::{HttpResponse, ResponseError};
	use derive_more::{Display, Error};

	#[derive(Debug, Display, Error)]
	enum UserError {
		#[display("An internal error occurred. Please try again later.")]
		InternalError,
	}

	impl ResponseError for UserError {
		fn status_code(&self) -> StatusCode {
			match *self {
				UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
			}
		}
		fn error_response(&self) -> HttpResponse {
			HttpResponse::build(self.status_code())
				.insert_header(ContentType::html())
				.body(self.to_string())
		}
	}

	async fn index() -> Result<&'static str, UserError> {
		do_things_that_fails().map_err(|e| UserError::InternalError)?;
		Ok("success!")
	}

	fn do_things_that_fails() -> Result<(), &'static str> {
		Err("test error")
	}
}

// [Error logging](https://actix.rs/docs/errors#error-logging-1)
pub mod example_6 {
	use actix_web::middleware::Logger;
	use actix_web::{App, HttpServer, ResponseError, get};
	use derive_more::{Display, Error};
	use log::info;

	#[derive(Debug, Display, Error)]
	pub struct MyError {
		#[display("my error: {name}")]
		name: &'static str,
	}

	// Use default implementation for `error_message()` method.
	impl ResponseError for MyError {}

	#[get("/")]
	async fn index() -> Result<&'static str, MyError> {
		let err = MyError { name: "test error" };
		info!("{}", err);
		Err(err)
	}

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		unsafe {
			std::env::set_var("RUST_LOG", "info");
			std::env::set_var("RUST_BACKTRACE", "1");
		}
		env_logger::init();

		HttpServer::new(|| {
			let logger = Logger::default();
			App::new().wrap(logger).service(index)
		})
		.bind(("localhost", 8080))?
		.run()
		.await
	}
}
