#[cfg(test)]
mod tests {
    use crate::modules::{tts::TextToSpeechClient, TextToSpeech};
    use std::error::Error;

    fn create_client() -> TextToSpeechClient {
        TextToSpeechClient
    }

    async fn assert_speak(text_or_file_path: &str, lang: &str) -> Result<(), Box<dyn Error>> {
        let client = create_client();
        client.speak(text_or_file_path.to_string(), lang).await?;
        Ok(())
    }

    #[actix_rt::test]
    async fn test_speak_polish() -> Result<(), Box<dyn Error>> {
        let result = assert_speak("stół z powyłamywanymi nogami", "pol").await;
        assert!(result.is_ok());
        Ok(())
    }

    #[actix_rt::test]
    async fn test_speak_english() -> Result<(), Box<dyn Error>> {
        let result = assert_speak("a table with broken legs", "eng").await;
        assert!(result.is_ok());
        Ok(())
    }

    #[actix_rt::test]
    async fn test_speak_italian() -> Result<(), Box<dyn Error>> {
        let result = assert_speak("un tavolo con le gambe rotte", "ita").await;
        assert!(result.is_ok());
        Ok(())
    }

    #[actix_rt::test]
    async fn test_speak_german() -> Result<(), Box<dyn Error>> {
        let result = assert_speak("ein Tisch mit gebrochenen Beinen", "ger").await;
        assert!(result.is_ok());
        Ok(())
    }

    #[actix_rt::test]
    async fn test_speak_spanish() -> Result<(), Box<dyn Error>> {
        let result = assert_speak("una mesa con patas rotas", "spa").await;
        assert!(result.is_ok());
        Ok(())
    }

    #[actix_rt::test]
    async fn test_speak_wrong_language() -> Result<(), Box<dyn Error>> {
        let result = assert_speak("une table avec des pieds cassés", "fra").await;
        assert!(result.is_ok());
        Ok(())
    }

    #[actix_rt::test]
    async fn test_blank_text() -> Result<(), Box<dyn Error>> {
        let result = assert_speak("", "pol").await;
        assert!(result.is_err());
        Ok(())
    }

    #[actix_rt::test]
    async fn test_wrong_file() -> Result<(), Box<dyn Error>> {
        let result = assert_speak("fake_text_file.txt", "eng").await;
        assert!(result.is_ok());
        Ok(())
    }
}
