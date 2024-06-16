use serde_json::{to_value, Value};
use std::collections::HashMap;

type ToolFunction = Box<dyn Fn(Value) -> Value + Send + Sync>;
type ToolRegistryEntry = (ToolFunction, Value);

pub struct ToolRegistry {
    tools: HashMap<String, ToolRegistryEntry>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register_tool<F, R>(&mut self, name: &str, func: F, schema: Value)
    where
        F: 'static + Send + Sync + Fn(Value) -> R,
        R: serde::Serialize,
    {
        let func = Box::new(move |params: Value| -> Value {
            let result: R = func(params);
            to_value(result).unwrap()
        });

        self.tools.insert(name.to_string(), (func, schema));
    }

    pub fn execute_tool(&self, name: &str, params: Value) -> Result<Value, String> {
        if let Some((func, _)) = self.tools.get(name) {
            Ok(func(params))
        } else {
            Err(format!("Tool {} not found", name))
        }
    }

    pub fn get_schema(&self, name: &str) -> Result<&Value, String> {
        if let Some((_, schema)) = self.tools.get(name) {
            Ok(schema)
        } else {
            Err(format!("Schema for tool {} not found", name))
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        ToolRegistry::new()
    }
}
