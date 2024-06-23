use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use async_trait::async_trait;
#[cfg(feature = "bedrock")]
use aws_sdk_bedrockruntime::types::{
    AutoToolChoice, Tool, ToolChoice, ToolConfiguration, ToolInputSchema, ToolSpecification,
};
#[cfg(feature = "bedrock")]
use aws_smithy_types::Document;
#[cfg(feature = "bedrock")]
use aws_smithy_types::Number;
use serde_json::{to_value, Value};
use tokio::sync::RwLock;

type ToolFunction = Arc<dyn Fn(Value) -> Value + Send + Sync>;

#[async_trait]
pub trait ToolHandler {
    async fn set_tool_registry(&mut self, tool_registry: &RwLock<ToolRegistry>);
}

pub struct ToolEntry<'a> {
    name: &'a str,
    description: &'a str,
    function: ToolFunction,
    spec: Value,
}

impl<'a> Debug for ToolEntry<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolEntry")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("spec", &self.spec)
            .finish()
    }
}

impl<'a> ToolEntry<'a> {
    pub fn new<F, R>(name: &'a str, description: &'a str, function: F, spec: Value) -> Self
    where
        F: 'static + Send + Sync + Fn(Value) -> R,
        R: serde::Serialize,
    {
        let func = Arc::new(move |params: Value| -> Value {
            let result: R = function(params);
            to_value(result).unwrap()
        });
        ToolEntry {
            name,
            description,
            function: func,
            spec,
        }
    }
}

impl<'a> Clone for ToolEntry<'a> {
    fn clone(&self) -> Self {
        ToolEntry {
            name: self.name,
            description: self.description,
            function: Arc::clone(&self.function),
            spec: self.spec.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToolRegistry<'a> {
    tools: HashMap<String, ToolEntry<'a>>,
}

impl<'a> ToolRegistry<'a> {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register_tool(&mut self, entry: ToolEntry<'a>) {
        self.tools.insert(entry.name.into(), entry);
    }

    pub fn execute_tool(&self, name: &str, params: Value) -> Result<Value, String> {
        if let Some(entry) = self.tools.get(name) {
            let func = &entry.function;
            Ok(func(params))
        } else {
            Err(format!("Tool {} not found", name))
        }
    }

    pub fn get_tool_spec(&self, name: &str) -> Result<&Value, String> {
        if let Some(entry) = self.tools.get(name) {
            Ok(&entry.spec)
        } else {
            Err(format!("Schema for tool {} not found", name))
        }
    }
}

impl<'a> Default for ToolRegistry<'a> {
    fn default() -> Self {
        ToolRegistry::new()
    }
}

#[cfg(feature = "bedrock")]
impl<'a> From<ToolRegistry<'a>> for ToolConfiguration {
    fn from(registry: ToolRegistry<'a>) -> ToolConfiguration {
        let tools: Vec<Tool> = registry
            .tools
            .into_values()
            .map(|entry| {
                let tool_spec = ToolSpecification::builder()
                    .name(entry.name.to_string())
                    .description(entry.description.to_string())
                    .input_schema(ToolInputSchema::Json(convert_value_to_document(
                        &entry.spec,
                    )))
                    .build()
                    .expect("Error constructing tool spec");
                Tool::ToolSpec(tool_spec)
            })
            .collect();

        ToolConfiguration::builder()
            .set_tools(Some(tools))
            .tool_choice(ToolChoice::Auto(AutoToolChoice::builder().build()))
            .build()
            .expect("Unable to build ToolConfiguration")
    }
}

#[cfg(feature = "bedrock")]
pub fn convert_value_to_document(value: &Value) -> Document {
    match value {
        Value::Null => Document::Null,
        Value::Bool(b) => Document::Bool(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into()
            } else if let Some(u) = n.as_u64() {
                u.into()
            } else if let Some(f) = n.as_f64() {
                f.into()
            } else {
                panic!("Unsupported number type")
            }
        }
        Value::String(s) => Document::String(s.clone()),
        Value::Array(arr) => Document::Array(arr.iter().map(convert_value_to_document).collect()),
        Value::Object(obj) => {
            let mut map = obj
                .iter()
                .map(|(k, v)| (k.clone(), convert_value_to_document(v)))
                .collect::<HashMap<String, Document>>();

            // Add the type field to the object map
            map.insert("type".to_string(), Document::String("object".to_string()));

            Document::Object(map)
        }
    }
}

#[cfg(feature = "bedrock")]
pub fn convert_document_to_value(doc: &Document) -> Value {
    match doc {
        Document::Null => Value::Null,
        Document::Bool(b) => Value::Bool(*b),
        Document::Number(n) => match n {
            Number::PosInt(u) => Value::Number(serde_json::Number::from(*u)),
            Number::NegInt(i) => Value::Number(serde_json::Number::from(*i)),
            Number::Float(f) => Value::Number(
                serde_json::Number::from_f64(*f)
                    .expect("Failed to convert float to serde_json::Number"),
            ),
        },
        Document::String(s) => Value::String(s.clone()),
        Document::Array(arr) => Value::Array(arr.iter().map(convert_document_to_value).collect()),
        Document::Object(obj) => {
            let mut map = obj
                .iter()
                .map(|(k, v)| (k.clone(), convert_document_to_value(v)))
                .collect::<serde_json::Map<String, Value>>();

            // Remove the type field from the object map if it exists
            map.remove("type");

            Value::Object(map)
        }
    }
}
