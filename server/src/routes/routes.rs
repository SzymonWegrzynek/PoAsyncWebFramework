use actix_web::web;

use crate::handlers::{
    healthcheck_handler::HealthCheck, parameter_handler::ParameterHandler,
    speech_handler::SpeechHandler,
};

pub fn healthcheck(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthcheck", web::get().to(HealthCheck::healthcheck));
}

pub fn speech(cfg: &mut web::ServiceConfig) {
    cfg.route("/to_speech", web::post().to(SpeechHandler::speech));
}

pub fn parameter(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/set_parameter",
        web::patch().to(ParameterHandler::set_parameter),
    );
}
