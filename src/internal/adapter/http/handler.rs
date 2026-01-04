use actix_web::{HttpResponse, web};

use crate::internal::infra::api::response;

async fn health() -> impl actix_web::Responder {
    HttpResponse::Ok().json(response::BaseResponse {
        message: "Service is running!".to_string(),
    })
}

pub fn routers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").route("/health", web::get().to(health)));
}
