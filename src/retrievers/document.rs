use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Option<String>,
    pub text: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: Option<serde_json::Value>,
}

impl From<String> for Document {
    fn from(text: String) -> Self {
        Self {
            id: None,
            text,
            embedding: None,
            metadata: None,
        }
    }
}
