use serde_json::{Value, json};
use std::error::Error;

pub enum Payload {
    Text(String),
    File(String),
}

impl Payload {
    pub fn to_json(&self) -> Result<Value, Box<dyn Error>> {
        let payload = match self {
            Payload::Text(t) => json!({
                "type": "text",
                "data": {
                    "text": t
                }
            }),
            Payload::File(f) => json!({
                "type": "file",
                "data": {
                    "file_path": f
                }
            }),
        };

        Ok(payload)
    }
}
