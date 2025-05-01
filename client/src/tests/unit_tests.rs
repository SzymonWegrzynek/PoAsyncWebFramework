#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::models::payload::Payload;

    #[test]
    fn test_text_payload_to_json() {
        let payload = Payload::Text("stół bez nóg".to_string());
        let json_value = payload.to_json().unwrap();

        let expected = json!({
            "type": "text",
            "data": {
                "text": "stół bez nóg"
            }
        });

        assert_eq!(json_value, expected);
    }

    #[test]
    fn test_file_payload_to_json() {
        let payload = Payload::File("example.txt".to_string());
        let json_value = payload.to_json().unwrap();

        let expected = json!({
            "type": "file",
            "data": {
                "file_path": "example.txt"
            }
        });

        assert_eq!(json_value, expected);
    }
}
