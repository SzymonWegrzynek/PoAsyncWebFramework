use actix_web::{web, HttpResponse};
use std::sync::{Arc, Mutex};

use crate::{models::parameter::ParameterRequest, state::AppState};

pub struct ParameterHandler;

impl ParameterHandler {
    pub async fn set_parameter(
        payload: web::Json<ParameterRequest>,
        app_state: web::Data<Arc<Mutex<AppState>>>,
    ) -> HttpResponse {
        let language = payload.language.clone();

        let mut state = app_state.lock().unwrap();
        state.lang = language;

        HttpResponse::Ok().finish()
    }
}
