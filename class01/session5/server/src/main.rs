use axum::routing::get;
use axum::{Extension, Json, Router};
use std::net::SocketAddr;

mod collector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Read .env
	dotenv::dotenv()?;
	let db_url = std::env::var("DATABASE_URL")?;
	let pool = sqlx::SqlitePool::connect(&db_url).await?;

	let handle = tokio::spawn(collector::data_collector(pool.clone()));

	// Launch Axum.
	let app = Router::new()
		.route("/api/all", get(show_all))
		.layer(Extension(pool));
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = tokio::net::TcpListener::bind(&addr).await?;
	axum::serve(listener, app).await?;

	// Wait for the data collector to finish.
	handle.await??;
	Ok(())
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct DataPoint {
	id: i32,
	collector_id: String,
	received: i64,
	total_memory: i64,
	used_memory: i64,
	average_cpu: f32,
}

pub async fn show_all(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<DataPoint>> {
	let rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries")
		.fetch_all(&pool)
		.await
		.unwrap();

	Json(rows)
}
