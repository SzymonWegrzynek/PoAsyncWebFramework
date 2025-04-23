use std::error::Error;

pub mod tts;

pub trait TextToSpeech {
    async fn speak(&self, text: String, language: &str) -> Result<(), Box<dyn Error>>;
}
