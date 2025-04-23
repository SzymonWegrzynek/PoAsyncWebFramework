use std::sync::Arc;

use crate::modules::tts::TextToSpeechClient;

#[derive(Clone)]
pub struct AppState {
    pub lang: String,
    pub tts_client: Arc<TextToSpeechClient>,
}
