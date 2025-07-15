// IKinder

// #![allow(unused)]

use axum::extract::path as axum_path;
use axum::http::header;
use futures::TryStreamExt;
use sqlx::Row;
use std::path as std_path;

const IMAGES: &'static str = "images";
const THUMB: &'static str = "_thumb";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Read the .env file and build environment variables.
	dotenv::dotenv()?;
	let db_url = std::env::var("DATABASE_URL")?;
	let pool = sqlx::SqlitePool::connect(&db_url).await?;

	// Run migrations.
	sqlx::migrate!("./migrations")
		.run(&pool)
		.await?;

	// Catch up on missing thumbnails.
	fill_missing_thumbnail(&pool).await?;

	// Build Axum with an "extension" to hold the database connection pool.
	let app = axum::Router::new()
		.route("/", axum::routing::get(index_page))
		.route("/test", axum::routing::get(test))
		.route("/upload", axum::routing::post(uploader))
		.route("/image/{id}", axum::routing::get(get_image))
		.route("/images", axum::routing::get(list_images))
		.route("/thumb/{id}", axum::routing::get(get_thumbnail))
		.route("/search", axum::routing::post(search_images))
		.layer(axum::Extension(pool));
	let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = tokio::net::TcpListener::bind(&addr).await?;
	axum::serve(listener, app).await?;

	Ok(())
}

async fn test(axum::Extension(pool): axum::Extension<sqlx::SqlitePool>) -> String {
	let result = sqlx::query("SELECT COUNT(id) FROM images")
		.fetch_one(&pool)
		.await
		.unwrap();
	let count = result.get::<i64, _>(0);
	format!("{} images in the database", count)
}

async fn index_page() -> axum::response::Html<String> {
	let path = std_path::Path::new("./html/index.html");
	let content = tokio::fs::read_to_string(path)
		.await
		.unwrap();
	axum::response::Html(content)
}

async fn uploader(
	axum::Extension(pool): axum::Extension<sqlx::SqlitePool>,
	mut multipart: axum::extract::Multipart,
) -> axum::response::Html<String> {
	let mut tags = None;
	let mut image = None;
	while let Some(field) = multipart.next_field().await.unwrap() {
		let name = field.name().unwrap().to_owned();
		let data = field.bytes().await.unwrap();

		match name.as_str() {
			"tags" => tags = Some(String::from_utf8(data.to_vec()).unwrap()),
			"image" => image = Some(data.to_vec()),
			_ => panic!("unknown field"),
		}
	}

	if let (Some(tags), Some(image)) = (tags, image) {
		let new_image_id = insert_image_into_db(&pool, &tags)
			.await
			.unwrap();
		save_image(new_image_id, &image)
			.await
			.unwrap();
		tokio::task::spawn_blocking(move || make_thumbnail(new_image_id))
			.await
			.unwrap()
			.unwrap();
	} else {
		panic!("missing field");
	}

	let path = std_path::Path::new("./html/redirect.html");
	tokio::fs::read_to_string(&path)
		.await
		.unwrap()
		.into()
}

async fn insert_image_into_db(pool: &sqlx::SqlitePool, tags: &str) -> anyhow::Result<i64> {
	let row = sqlx::query("INSERT INTO images (tags) VALUES (?) RETURNING id")
		.bind(tags)
		.fetch_one(pool)
		.await?;
	Ok(row.get(0))
}

async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
	// Check that the image folder exists and is a directory.
	// Create it if it doesn't.
	let base_path = std::path::Path::new(IMAGES);
	if !base_path.exists() || !base_path.is_dir() {
		tokio::fs::create_dir_all(base_path).await?;
	}

	// Use "join" to create a path to the image file. Join is platform aware,
	// it will handle differences between Windows and Unix.
	let image_path = base_path.join(format!("{id}.jpg"));
	if image_path.exists() {
		// The file exsists. That couldn't happen.
		anyhow::bail!("File already exists");
	}

	// Write the image to the file.
	tokio::fs::write(image_path, bytes).await?;
	Ok(())
}

async fn get_image(axum_path::Path(id): axum_path::Path<i64>) -> impl axum::response::IntoResponse {
	let filename = format!("{}/{}.jpg", IMAGES, id);
	get_image_impl(filename).await
}

fn make_thumbnail(id: i64) -> anyhow::Result<()> {
	let image_path = format!("{}/{}.jpg", IMAGES, id);
	let thumbnail_path = format!("{}/{}{}.jpg", IMAGES, id, THUMB);
	let image_bytes = std::fs::read(image_path)?;

	let image = if let Ok(format) = image::guess_format(&image_bytes) {
		image::load_from_memory_with_format(&image_bytes, format)?
	} else {
		image::load_from_memory(&image_bytes)?
	};

	image
		.thumbnail(64, 64)
		.save(thumbnail_path)?;

	Ok(())
}

async fn fill_missing_thumbnail(pool: &sqlx::Pool<sqlx::Sqlite>) -> anyhow::Result<()> {
	let mut rows = sqlx::query("SELECT id FROM images").fetch(pool);

	while let Some(row) = rows.try_next().await? {
		let id = row.get(0);
		let thumbnail_path = format!("{}/{}{}.jpg", IMAGES, id, THUMB);

		if !std_path::Path::new(&thumbnail_path).exists() {
			tokio::task::spawn_blocking(move || make_thumbnail(id)).await??;
		}
	}

	Ok(())
}

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
struct ImageRecord {
	id: i64,
	tags: String,
}

async fn list_images(axum::Extension(pool): axum::Extension<sqlx::SqlitePool>) -> axum::Json<Vec<ImageRecord>> {
	sqlx::query_as::<_, ImageRecord>("SELECT * FROM images ORDER BY id")
		.fetch_all(&pool)
		.await
		.unwrap()
		.into()
}

async fn get_thumbnail(axum_path::Path(id): axum_path::Path<i64>) -> impl axum::response::IntoResponse {
	let filename = format!("{}/{}{}.jpg", IMAGES, id, THUMB);
	get_image_impl(filename).await
}

async fn get_image_impl(filename: String) -> impl axum::response::IntoResponse {
	let attachment = format!("filename={}", filename);
	let file = tokio::fs::File::open(&filename)
		.await
		.unwrap();

	axum::response::Response::builder()
		.header(header::CONTENT_TYPE, header::HeaderValue::from_static("image/jpeg"))
		.header(header::CONTENT_DISPOSITION, header::HeaderValue::from_str(&attachment).unwrap())
		.body(axum::body::Body::from_stream(tokio_util::io::ReaderStream::new(file)))
		.unwrap()
}

#[derive(serde::Deserialize)]
struct SearchResult {
	tags: String,
}

async fn search_images(
	axum::Extension(pool): axum::Extension<sqlx::SqlitePool>,
	axum::Form(form): axum::Form<SearchResult>,
) -> axum::response::Html<String> {
	let tag = format!("%{}%", form.tags);
	let rows = sqlx::query_as::<_, ImageRecord>("SELECT * FROM images WHERE tags LIKE ? ORDER BY id")
		.bind(tag)
		.fetch_all(&pool)
		.await
		.unwrap();

	let mut results = String::new();
	for row in rows {
		results.push_str(&format!(
			"<a href=\"/image/{}\"><img src='/thumb/{}' /></a><br />",
			row.id, row.id
		));
	}

	let path = std_path::Path::new("./html/search.html");
	let mut content = tokio::fs::read_to_string(path)
		.await
		.unwrap();
	content = content.replace("{results}", &results);
	axum::response::Html(content)
}
