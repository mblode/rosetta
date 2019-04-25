extern crate actix_web;
use actix_web::{http, server, App, Path, Responder};

fn index(info: Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

pub fn serve_site (source_dir: &str, destination_dir: &str) {
    server::new(
        || App::new()
            .route("/{id}/{name}/index.html", http::Method::GET, index))
        .bind("127.0.0.1:8080").unwrap()
        .run();
}
