use axum::extract::Path;
use axum::response::Html;
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
		.route("/", get(index))
		.route("/collector.html", get(collector))
		.route("/api/all", get(show_all))
		.route("/api/collectors", get(show_collectors))
		.route("/api/collector/{uuid}", get(collector_data))
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

pub async fn index() -> Html<String> {
	let path = std::path::Path::new("./html/index.html");
	let content = tokio::fs::read_to_string(path)
		.await
		.unwrap();
	Html(content)
}

pub async fn collector() -> Html<String> {
	let path = std::path::Path::new("./html/collector.html");
	let content = tokio::fs::read_to_string(path)
		.await
		.unwrap();
	Html(content)
}

pub async fn show_all(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<DataPoint>> {
	let rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries")
		.fetch_all(&pool)
		.await
		.unwrap();

	Json(rows)
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct Collector {
	id: i32,
	collector_id: String,
	last_seen: i64,
}

pub async fn show_collectors(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<Collector>> {
	let collectors = sqlx::query_as::<_, Collector>(
		r#"
		SELECT
			DISTINCT(id) AS id,
			collector_id,
			(SELECT MAX(received)
			 FROM timeseries
			 WHERE collector_id = ts.collector_id) AS last_seen
		FROM
			timeseries ts
		"#,
	)
	.fetch_all(&pool)
	.await
	.unwrap();

	Json(collectors)
}

pub async fn collector_data(Extension(pool): Extension<sqlx::SqlitePool>, uuid: Path<String>) -> Json<Vec<DataPoint>> {
	let rows = sqlx::query_as::<_, DataPoint>(
		r#"
		SELECT * FROM timeseries
		WHERE collector_id = ?
		ORDER BY received
		"#,
	)
	.bind(uuid.as_str())
	.fetch_all(&pool)
	.await
	.unwrap();

	Json(rows)
}
