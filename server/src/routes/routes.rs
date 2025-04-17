use actix_web::web;

use crate::handlers::{healthcheck_handler::HealthCheck, speech_handler::SpeechHandler};

pub fn healthcheck(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthcheck", web::get().to(HealthCheck::healthcheck));
}

pub fn speech(cfg: &mut web::ServiceConfig) {
    cfg.route("/to_speech", web::get().to(SpeechHandler::speech));
}
