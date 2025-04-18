use std::error::Error;
use tts_rust::{languages::Languages, tts::GTTSClient};

pub struct Tts;

impl Tts {
    pub async fn synthesize(text: String, volume: f32) -> Result<(), Box<dyn Error>> {
        let narrator: GTTSClient = GTTSClient {
            volume: volume, // od 0.0 do 1.0
            language: Languages::Polish,
            tld: "com",
        };

        let _ = narrator.save_to_file(&text, "audio.mp3");

        Ok(())
    }
}
