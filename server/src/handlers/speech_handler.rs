use actix_web::{web, HttpResponse};
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};

use crate::{models::speech::SpeechRequest, modules::TextToSpeech, state::AppState};

pub struct SpeechHandler;

impl SpeechHandler {
    pub async fn speech(
        payload: web::Json<SpeechRequest>,
        app_state: web::Data<Arc<Mutex<AppState>>>,
    ) -> HttpResponse {
        let text: String = match payload.into_inner() {
            SpeechRequest::Text { text } => text,
            SpeechRequest::File { file_path } => match read_to_string(&file_path) {
                Ok(file_content) => file_content,
                Err(_) => return HttpResponse::BadRequest().body("Failed to read the file"),
            },
        };

        let state = app_state.lock().unwrap();
        let tts = state.tts_client.clone();

        match tts.speak(text, &state.lang).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => {
                return HttpResponse::InternalServerError().body("Tts error");
            }
        }
    }
}
