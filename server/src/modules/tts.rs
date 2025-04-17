use std::error::Error;
use tts_rust::{languages::Languages, tts::GTTSClient};

pub struct Tts;

impl Tts {
    pub async fn synthesize(text: String) -> Result<(), Box<dyn Error>> {
        let narrator: GTTSClient = GTTSClient {
            volume: 1.0, // od 0.0 do 1.0
            language: Languages::Polish,
            tld: "com",
        };

        let _ = narrator.speak(&text);

        Ok(())
    }
}
