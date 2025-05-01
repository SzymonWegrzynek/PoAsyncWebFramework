#[cfg(test)]
mod tests {
    use crate::modules::{tts::TextToSpeechClient, TextToSpeech};
    use std::error::Error;

    fn create_client() -> TextToSpeechClient {
        TextToSpeechClient
    }

    async fn assert_speak(text: &str, lang: &str, outcome: bool) -> Result<(), Box<dyn Error>> {
        let client = create_client();
        let result = client.speak(text.to_string(), lang).await;

        if outcome {
            assert!(result.is_ok());
        } else {
            assert!(result.is_err());
        }

        Ok(())
    }

    #[actix_rt::test]
    async fn test_speak_polish() -> Result<(), Box<dyn Error>> {
        assert_speak("stół z powyłamywanymi nogami", "pol", true).await
    }

    #[actix_rt::test]
    async fn test_speak_english() -> Result<(), Box<dyn Error>> {
        assert_speak("a table with broken legs", "eng", true).await
    }

    #[actix_rt::test]
    async fn test_speak_italian() -> Result<(), Box<dyn Error>> {
        assert_speak("un tavolo con le gambe rotte", "ita", true).await
    }

    #[actix_rt::test]
    async fn test_speak_german() -> Result<(), Box<dyn Error>> {
        assert_speak("ein Tisch mit gebrochenen Beinen", "ger", true).await
    }

    #[actix_rt::test]
    async fn test_speak_spanish() -> Result<(), Box<dyn Error>> {
        assert_speak("una mesa con patas rotas", "spa", true).await
    }

    #[actix_rt::test]
    async fn test_speak_wrong_language() -> Result<(), Box<dyn Error>> {
        assert_speak("une table avec des pieds cassés", "fra", true).await
    }

    #[actix_rt::test]
    async fn test_blank_text() -> Result<(), Box<dyn Error>> {
        assert_speak("", "", false).await
    }

    #[actix_rt::test]
    async fn test_wrong_file() -> Result<(), Box<dyn Error>> {
        assert_speak("./text.txt", "", false).await
    }
}
