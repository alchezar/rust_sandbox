use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub mod lib {}
pub mod web;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Anyhow: {0}")]
	Anyhow(#[from] anyhow::Error),
	#[error("Error: {0}")]
	Any(String),
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED").into_response()
	}
}

pub fn show_name(file: &str) -> Option<()> {
	println!(
		"class10::{}\n",
		file.split("\\")
			.last()?
			.split(".")
			.next()?
	);
	Some(())
}
