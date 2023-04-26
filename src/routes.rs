// src/routes.rs
use actix_web::web;
use crate::bank_controller::get_banks;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_banks)
    );
}
