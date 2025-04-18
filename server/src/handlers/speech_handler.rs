use actix_web::{web, HttpResponse};
use std::fs::read_to_string;

use crate::{models::speech::SpeechRequest, modules::tts::Tts};

pub struct SpeechHandler;

impl SpeechHandler {
    pub async fn speech(payload: web::Json<SpeechRequest>) -> HttpResponse {
        let content = match payload.into_inner() {
            SpeechRequest::Text { text } => Ok(text),
            SpeechRequest::File { file_path } => read_to_string(file_path)
                .map_err(|_| HttpResponse::BadRequest().body("Failed to read the file")),
        };

        let text = match content {
            Ok(text) => text,
            Err(_) => {
                return HttpResponse::BadRequest().body("Invalid speech request");
            }
        };

        match Tts::synthesize(text, 1.0).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => {
                return HttpResponse::InternalServerError().body("Tts error");
            }
        }
    }
}
