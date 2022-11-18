use actix_web::http::header;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::common::dependency_injection::ServicesInjected;

mod common;
mod controllers;
mod errors;

pub async fn serve() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let server = HttpServer::new(move || {
        let authentication_mw = HttpAuthentication::bearer(common::http_bearer_authentication);
        let default_header_mw = middleware::DefaultHeaders::new().add((
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        ));

        App::new()
            .app_data(web::Data::new(ServicesInjected::default()))
            .wrap(default_header_mw)
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Compress::default())
            .configure(public_routes)
            .service(
                web::scope("")
                    .wrap(authentication_mw)
                    .configure(private_routes),
            )
    });

    println!("Server started");
    server.bind(("127.0.0.1", 7000))?.run().await
}

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .configure(controllers::authentication_controller::routes);
}

pub fn private_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(controllers::dinners_controller::routes);
}

#[get("/hello")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("hello word")
}
