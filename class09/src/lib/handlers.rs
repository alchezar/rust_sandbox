// [Request Handlers](https://actix.rs/docs/handlers/)
pub mod example_1 {
    use actix_web::{HttpRequest, HttpResponse, Responder, web};

    async fn index_1(_req: HttpRequest) -> &'static str {
        "Hello world!"
    }

    async fn index_2(_req: HttpRequest) -> String {
        "Hello world!".to_owned()
    }

    async fn index_3(_req: HttpRequest) -> impl Responder {
        web::Bytes::from_static(b"Hello world!")
    }

    async fn index_4(_req: HttpRequest) -> std::io::Result<HttpResponse> {
        Ok(HttpResponse::Ok().body("Hello world!"))
    }
}

// [Response with custom type](https://actix.rs/docs/handlers/#response-with-custom-type)
pub mod example_2 {
    use actix_web::body::{BoxBody, MessageBody};
    use actix_web::{HttpRequest, HttpResponse, Responder};

    #[derive(serde::Serialize)]
    struct MyObj {
        name: &'static str,
    }

    // Responder
    impl Responder for MyObj {
        type Body = BoxBody;
        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            HttpResponse::Ok().body(serde_json::to_string(&self).unwrap())
        }
    }

    async fn index() -> impl Responder {
        MyObj { name: "user" }
    }
}

// [Streaming response body](https://actix.rs/docs/handlers/#streaming-response-body)
pub mod example_3 {
    use actix_files as af;
    use actix_web as aw;
    use aw::web;
    use futures::future::ok;
    use futures::stream::{self, Stream, once};

    #[aw::main]
    async fn main() -> std::io::Result<()> {
        aw::HttpServer::new(|| aw::App::new().service(stream_func))
            .bind("localhost:8080")?
            .run()
            .await
    }

    #[aw::get("/stream")]
    async fn stream_func() -> impl aw::Responder {
        let body = once(ok::<_, aw::Error>(web::Bytes::from_static(b"test")));

        aw::HttpResponse::Ok()
            .content_type("application/json")
            .streaming(body)
    }

    // Other examples
    #[aw::get("/simple-stream")]
    async fn simple_stream() -> aw::HttpResponse {
        let data = vec!["first chunk\n", "second chunk\n", "third chunk\n"];

        let data_stream = stream::iter(
            data.into_iter()
                .map(|chunk| Ok::<_, aw::Error>(web::Bytes::from(chunk))),
        );

        aw::HttpResponse::Ok()
            .content_type("text/plain")
            .streaming(data_stream)
    }
}

// [Different return types (Either)](https://actix.rs/docs/handlers/#different-return-types-either)
pub mod example_4 {
    use actix_web as aw;

    type RegisterResult = aw::Either<aw::HttpResponse, aw::Result<&'static str, aw::Error>>;

    fn is_a_variant() -> bool {
        false
    }

    async fn index() -> RegisterResult {
        if is_a_variant() {
            aw::Either::Left(aw::HttpResponse::BadRequest().body("Bad data"))
        } else {
            aw::Either::Right(Ok("Hello world!"))
        }
    }
}
