//! Structures for managing documents in vector databases.
//!
//! This module provides the `Document` and `DocCollection` structs for handling and managing
//! documents in vector databases.

use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};

const DEFAULT_EMBEDDING_NAME: &str = "embedding";

/// Document structure for serializing and deserializing when working with vector databases.
///
/// The `id` field is a unique identifier for the document. If not provided, it will be generated
/// using a hash of the `text` field. The `text` field is the main content of the document. The
/// `embedding` field is an optional field that can be used to store a vector embedding of the
/// document. The `embedding_name` field is the name of the field that the embedding is stored in.
/// The `metadata` field is an optional field that can be used to store additional metadata about
/// the document.
#[derive(Clone)]
pub struct Document {
    pub id: String,
    pub text: String,
    pub embedding: Option<Vec<f32>>,
    pub embedding_name: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl Document {
    /// Generate a unique identifier for a document based on its text.
    fn hash_text(text: &str) -> String {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();
        BASE64_URL_SAFE_NO_PAD.encode(hash.to_be_bytes())
    }

    /// Create a new document with the given text.
    pub fn new(text: String) -> Self {
        Self {
            id: Self::hash_text(&text),
            text,
            embedding: None,
            embedding_name: None,
            metadata: None,
        }
    }

    /// Create a new document with the given id and text.
    #[allow(dead_code)]
    pub fn new_with_id(id: String, text: String) -> Self {
        Self {
            id,
            text,
            embedding: None,
            embedding_name: None,
            metadata: None,
        }
    }

    /// Create a new document with the given text and embedding.
    #[allow(dead_code)]
    pub fn new_with_embedding(text: String, embedding: Vec<f32>, embedding_name: String) -> Self {
        Self {
            id: Self::hash_text(&text),
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

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let embedding_preview = match &self.embedding {
            Some(vec) if !vec.is_empty() => {
                let preview = vec.iter().take(2).collect::<Vec<_>>();
                // Output [preview[0], preview[1], ... (n more)]
                let mut preview_str = format!("[{}", preview[0]);
                if vec.len() > 1 {
                    preview_str.push_str(&format!(", {}", preview[1]));
                }
                if vec.len() > 2 {
                    preview_str.push_str(&format!(", ...({} more)", vec.len() - 2));
                }
                preview_str.push(']');
                preview_str
            }
            Some(_) => "[]".to_string(),
            None => "None".to_string(),
        };

        write!(
            f,
            "Document {{ id: {:?}, text: {:?}, embedding: {}, embedding_name: {:?}, metadata: {:?} }}",
            self.id, self.text, embedding_preview, self.embedding_name, self.metadata
        )
    }
}

/// A struct representing a collection of documents.
#[allow(dead_code)]
pub struct DocCollection {
    documents: Vec<Document>,
}

impl<T: Into<Document>> FromIterator<T> for DocCollection {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            documents: iter.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Vec<String>> for DocCollection {
    fn from(texts: Vec<String>) -> Self {
        Self {
            documents: texts.into_iter().map(Document::from).collect(),
        }
    }
}

impl From<DocCollection> for Vec<Document> {
    fn from(docs: DocCollection) -> Self {
        docs.documents
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

        doc["id"] = serde_json::json!(self.id);

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
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let text = doc
            .get("text")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| serde::de::Error::missing_field("text"))?;

        let id = id.unwrap_or_else(|| Document::hash_text(&text));
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
            id: "1".to_string(),
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
            "id": "1",
            "text": "hello",
            "embedding": [1.0, 2.0, 3.0],
            "metadata": {"key": "value", "embedding_field_name": "embedding"}
        }"#;

        let deserialized: super::Document = serde_json::from_str(serialized).unwrap();

        assert_eq!(deserialized.id, "1".to_string());
        assert_eq!(deserialized.text, "hello");
        assert_eq!(deserialized.embedding, Some(vec![1.0, 2.0, 3.0]));
        assert_eq!(deserialized.embedding_name, Some("embedding".to_string()));
        assert_eq!(
            deserialized.metadata,
            Some(serde_json::json!({"embedding_field_name": "embedding", "key": "value"}))
        );
    }
}
