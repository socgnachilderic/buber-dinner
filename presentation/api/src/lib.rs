use actix_web::{get, web, App, HttpResponse, HttpServer};
use application::services::authentication::AuthenticationService;
use infrastructure::{
    authentication::{jwt_settings::JwtSettings, jwt_token_generator::JwtTokenGenerator},
    services::date_provider::DateTimeProvider,
};

mod controllers;

struct ServicesInjected {
    pub authentication: AuthenticationService,
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

        let datetime_provider = Box::new(DateTimeProvider);
        let jwt_token_generator = Box::new(JwtTokenGenerator::new(jwt_settings, datetime_provider));
        let authentication_service = AuthenticationService::new(jwt_token_generator);

        Self {
            authentication: authentication_service,
        }
    }
}

pub async fn serve() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ServicesInjected::default()))
            .configure(routes)
            .service(hello)
    });

    println!("Server started");
    server.bind(("127.0.0.1", 7000))?.run().await
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
