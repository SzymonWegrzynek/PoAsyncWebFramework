use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ParameterRequest {
    pub language: String,
}
