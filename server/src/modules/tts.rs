use std::error::Error;
use tts_rust::{languages::Languages, tts::GTTSClient};

use super::TextToSpeech;

pub struct TextToSpeechClient;

impl TextToSpeech for TextToSpeechClient {
    async fn speak(&self, text: String, language: &str) -> Result<(), Box<dyn Error>> {
        let narrator: GTTSClient = match language {
            "pol" => GTTSClient {
                volume: 1.0,
                language: Languages::Polish,
                tld: "com",
            },

            "eng" => GTTSClient {
                volume: 1.0,
                language: Languages::English,
                tld: "com",
            },

            "ita" => GTTSClient {
                volume: 1.0,
                language: Languages::Italian,
                tld: "com",
            },

            "ger" => GTTSClient {
                volume: 1.0,
                language: Languages::German,
                tld: "com",
            },

            "spa" => GTTSClient {
                volume: 1.0,
                language: Languages::Spanish,
                tld: "com",
            },

            _ => GTTSClient {
                volume: 1.0,
                language: Languages::Polish,
                tld: "com",
            },
        };

        narrator.save_to_file(&text, "/mnt/c/Users/super/Downloads/audio.mp3")?;

        Ok(())
    }
}
