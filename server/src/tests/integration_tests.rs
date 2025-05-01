#[cfg(test)]
mod tests {
    use actix_web::{
        http::{header, StatusCode},
        test, web, App,
    };
    use serde_json::{json, Value};
    use std::sync::{Arc, Mutex};

    use crate::{
        handlers::speech_handler::SpeechHandler, modules::tts::TextToSpeechClient, AppState,
    };

    async fn get_speech_endpoint(payload: Value) -> StatusCode {
        let app_state = Arc::new(Mutex::new(AppState {
            lang: "pol".to_string(),
            tts_client: Arc::new(TextToSpeechClient),
        }));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state.clone()))
                .route("/to_speech", web::post().to(SpeechHandler::speech)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/to_speech")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(payload.to_string())
            .to_request();

        let resp = test::call_service(&app, req).await;

        resp.status()
    }

    #[actix_rt::test]
    async fn test_speech_endpint_text() {
        let payload = json!({
            "type": "text",
            "data": {
                "text": "una mesa con patas rotas"
            }
        });

        let status = get_speech_endpoint(payload).await;

        assert_eq!(status, StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_speech_endpint_file_200() {
        let payload = json!({
            "type": "file",
            "data": {
                "file_path": "/home/szymon/projects/po-async-web-framework/example.txt"
            }
        });

        let status = get_speech_endpoint(payload).await;

        assert_eq!(status, StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_speech_endpint_file_400() {
        let payload = json!({
            "type": "file",
            "data": {
                "file_path": "fake_text_file.txt"
            }
        });

        let status = get_speech_endpoint(payload).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_speech_endpint_500() {
        let payload = json!({
            "type": "file",
            "data": {
                "file_path": "/home/szymon/projects/po-async-web-framework/readme.md"
            }
        });

        let status = get_speech_endpoint(payload).await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
