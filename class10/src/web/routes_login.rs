use crate::{Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};

pub fn routes() -> Router {
	Router::new()
		// curl --location http://localhost:6666/api/login --header 'Content-Type: application/json' --data '{"username": "user", "password": "pass"}'
		.route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	password: String,
}

async fn api_login(Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
	println!("->> {:<12} - api_login", "HANDLER");

	if payload.username != "user" || payload.password != "pass" {
		return Err(Error::Any("Login fail.".to_owned()));
	}

	let body = Json(json!({"result": {"success": true}}));
	Ok(body)
}
