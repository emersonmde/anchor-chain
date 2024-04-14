use serde::{Deserialize, Deserializer, Serialize, Serializer};

const DEFAULT_EMBEDDING_NAME: &str = "embedding";

#[derive(Debug, Clone)]
pub struct Document {
    pub id: Option<String>,
    pub text: String,
    pub embedding: Option<Vec<f32>>,
    pub embedding_name: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl Document {
    pub fn new(text: String) -> Self {
        Self {
            id: None,
            text,
            embedding: None,
            embedding_name: None,
            metadata: None,
        }
    }

    pub fn new_with_id(id: String, text: String) -> Self {
        Self {
            id: Some(id),
            text,
            embedding: None,
            embedding_name: None,
            metadata: None,
        }
    }

    pub fn new_with_embedding(text: String, embedding: Vec<f32>, embedding_name: String) -> Self {
        Self {
            id: None,
            text,
            embedding: Some(embedding),
            embedding_name: Some(embedding_name),
            metadata: None,
        }
    }
}

impl From<String> for Document {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl Serialize for Document {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut doc = serde_json::json!({
            "text": self.text,
        });

        if let Some(id) = &self.id {
            doc["_id"] = serde_json::json!(id);
        }

        if let Some(embedding) = &self.embedding {
            let embedding_field_name = self
                .embedding_name
                .as_deref()
                .unwrap_or(DEFAULT_EMBEDDING_NAME);
            doc[embedding_field_name] = serde_json::json!(embedding);

            let mut metadata = self.metadata.clone().unwrap_or_default();
            metadata["embedding_field_name"] = serde_json::json!(embedding_field_name);
            doc["metadata"] = metadata;
        } else if let Some(metadata) = &self.metadata {
            doc["metadata"] = metadata.clone();
        }

        doc.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Document {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let doc = serde_json::Value::deserialize(deserializer)?;

        let id = doc
            .get("_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let text = doc
            .get("text")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| serde::de::Error::missing_field("text"))?;

        let metadata = doc.get("metadata").cloned();

        let embedding_name = metadata
            .as_ref()
            .and_then(|m| m.get("embedding_field_name"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let embedding = if let Some(name) = &embedding_name {
            doc.get(name)
                .and_then(|v| serde_json::from_value(v.clone()).ok())
        } else {
            None
        };

        Ok(Document {
            id,
            text,
            embedding,
            embedding_name,
            metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_document_serialization() {
        let doc = super::Document {
            id: Some("1".to_string()),
            text: "hello".to_string(),
            embedding: Some(vec![1.0, 2.0, 3.0]),
            embedding_name: Some("embedding".to_string()),
            metadata: Some(serde_json::json!({"key": "value"})),
        };

        let serialized = serde_json::to_string(&doc).unwrap();
        let deserialized: super::Document = serde_json::from_str(&serialized).unwrap();

        assert_eq!(doc.id, deserialized.id);
        assert_eq!(doc.text, deserialized.text);
        assert_eq!(doc.embedding, deserialized.embedding);
        assert_eq!(doc.embedding_name, deserialized.embedding_name);
        assert_eq!(
            doc.metadata.map(|mut m| {
                m["embedding_field_name"] = serde_json::json!("embedding");
                m
            }),
            deserialized.metadata
        );
    }

    #[test]
    fn test_document_deserialization() {
        let serialized = r#"{
            "_id": "1",
            "text": "hello",
            "embedding": [1.0, 2.0, 3.0],
            "metadata": {"key": "value", "embedding_field_name": "embedding"}
        }"#;

        let deserialized: super::Document = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.id, Some("1".to_string()));
        assert_eq!(deserialized.text, "hello");
        assert_eq!(deserialized.embedding, Some(vec![1.0, 2.0, 3.0]));
        assert_eq!(deserialized.embedding_name, Some("embedding".to_string()));
        assert_eq!(
            deserialized.metadata,
            Some(serde_json::json!({"embedding_field_name": "embedding", "key": "value"}))
        );
    }
}
