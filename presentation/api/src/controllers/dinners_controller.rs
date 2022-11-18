use actix_web::{web, HttpResponse, Responder, Result};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/dinners", web::get().to(list_dinners));
    // .service(web::scope("/dinners").service(list_dinners));
}

async fn list_dinners() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(Vec::<String>::new()))
}
