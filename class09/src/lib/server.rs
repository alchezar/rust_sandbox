pub mod example_1 {
	use actix_web::{App, HttpResponse, HttpServer, web};

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		HttpServer::new(|| App::new().route("/", web::get().to(|| async { HttpResponse::Ok() })))
			.bind("localhost:8080")?
			.run()
			.await
	}
}

pub mod example_2 {
	use actix_web::{App, HttpResponse, HttpServer, Responder, web};

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		HttpServer::new(|| App::new().route("/", web::get().to(|| async { HttpResponse::Ok() })))
			.bind("localhost:8080")?
			.workers(4)
			.run()
			.await
	}

	fn bad_handler() -> impl Responder {
		std::thread::sleep(std::time::Duration::from_secs(5));
		"response"
	}

	async fn good_handler() -> impl Responder {
		tokio::time::sleep(std::time::Duration::from_secs(5)).await;
		"response"
	}
}

pub mod example_3 {
	use actix_web::{App, HttpServer, Responder, get};
	// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		// let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
		// builder.set_private_key_file("key.pem", SslFiletype::PEM);
		// builder.set_certificate_chain_file("cert.pem");

		HttpServer::new(|| App::new().service(index))
			// .bind_openssl("localhost:8080", builder)?
			.run()
			.await
	}

	#[get("/")]
	async fn index() -> impl Responder {
		"Welcome!"
	}
}

pub mod example_4 {
	use actix_web::http::{ConnectionType, KeepAlive};
	use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
	use std::time::Duration;

	#[actix_web::main]
	async fn main() -> std::io::Result<()> {
		// Set keep-alive to 75 seconds.
		let one = HttpServer::new(|| App::new()).keep_alive(Duration::from_secs(75));
		// Use OS's keep-alive (usually quite long).
		let two = HttpServer::new(|| App::new()).keep_alive(KeepAlive::Os);
		// Disable keep-alive
		let three = HttpServer::new(|| App::new())
			.keep_alive(None)
			.shutdown_timeout(60);

		Ok(())
	}

	async fn index(_req: HttpRequest) -> impl Responder {
		let mut resp = HttpResponse::Ok()
			.force_close()
			.finish();
		resp.head_mut()
			.set_connection_type(ConnectionType::Close);
		resp
	}
}
