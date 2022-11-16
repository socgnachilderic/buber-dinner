use std::sync::Arc;

use actix_web::http::header;
use actix_web::middleware::DefaultHeaders;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use application::authentication::commands::AuthenticationCommandHandler;
use application::authentication::queries::AuthenticationQueryHandler;
use infrastructure::authentication::jwt_settings::JwtSettings;
use infrastructure::authentication::jwt_token_generator::JwtTokenGenerator;
use infrastructure::persistence::user_inmemory_repository::UserInMemoryRepository;
use infrastructure::services::date_provider::DateTimeProvider;

mod controllers;
mod errors;
mod middlewares;

pub async fn serve() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let server = HttpServer::new(move || {
        let default_header_mw = DefaultHeaders::new().add((
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        ));

        App::new()
            .app_data(web::Data::new(ServicesInjected::default()))
            .wrap(default_header_mw)
            .configure(routes)
            .service(hello)
    });

    println!("Server started");
    server.bind(("127.0.0.1", 7000))?.run().await
}

struct ServicesInjected {
    pub authentication_command: AuthenticationCommandHandler,
    pub authentication_query: AuthenticationQueryHandler,
}

impl Default for ServicesInjected {
    fn default() -> Self {
        let env = Env::new();
        let jwt_settings = JwtSettings::new(
            env.jwt_secret,
            env.jwt_expiry_minutes,
            env.jwt_issuer,
            env.jwt_audience,
        );

        let datetime_provider = Arc::new(DateTimeProvider);
        let jwt_token_generator = Arc::new(JwtTokenGenerator::new(jwt_settings, datetime_provider));
        let user_repository = Arc::new(UserInMemoryRepository);

        Self {
            authentication_command: AuthenticationCommandHandler::new(
                jwt_token_generator.clone(),
                user_repository.clone(),
            ),
            authentication_query: AuthenticationQueryHandler::new(
                jwt_token_generator,
                user_repository,
            ),
        }
    }
}

#[derive(Debug)]
struct Env {
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub jwt_expiry_minutes: u64,
}

impl Env {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        Self {
            jwt_secret: Self::get_var("JWT_SECRET"),
            jwt_issuer: Self::get_var("JWT_ISSUER"),
            jwt_audience: Self::get_var("JWT_AUDIENCE"),
            jwt_expiry_minutes: Self::get_var("JWT_EXPIRY_MINUTES").parse::<u64>().unwrap(),
        }
    }

    fn get_var(key: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| panic!("{key} not found."))
    }
}

#[get("/hello")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("hello word")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(controllers::authentication_controller::routes);
}
