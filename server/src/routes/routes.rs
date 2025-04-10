use actix_web::web;

use crate::handlers::healthcheck_handler::HealthCheck;

pub fn healthcheck(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthcheck", web::get().to(HealthCheck::healthcheck));
}
