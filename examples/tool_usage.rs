use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;
use serde_json::Value;
use tokio::sync::RwLock;

use anchor_chain::{AgentExecutor, ChainBuilder, ToolRegistry};
use anchor_chain_macros::tool;

static TOOL_REGISTRY: Lazy<RwLock<ToolRegistry>> = Lazy::new(|| RwLock::new(ToolRegistry::new()));

/// Generates the current weather in Celsius
///
/// # Parameters
/// - None
///
/// # Returns
/// - A float representing the current temperature in Celsius.
#[tool(TOOL_REGISTRY)]
fn get_weather() -> f64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seed = now.as_secs();

    (seed % 40) as f64
}

/// Converts a temperature from Celsius to Fahrenheit
///
/// # Parameters
/// - `celsius`: A float representing the temperature in Celsius.
///
/// # Returns
/// - A float representing the temperature in Fahrenheit.
#[tool(TOOL_REGISTRY)]
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

/// Provides a common sentiment based on the temperature
///
/// # Parameters
/// - `temp`: A float representing the temperature in Fahrenheit.
///
/// # Returns
/// - A `String` with a sentiment on the temperature.
#[tool(TOOL_REGISTRY)]
fn weather_sentiment(temp: f64) -> String {
    if temp > 85.0 {
        format!("{temp} is hot").to_string()
    } else if temp < 60.0 {
        format!("{temp} is cold").to_string()
    } else {
        format!("{temp} is moderate").to_string()
    }
}

#[tokio::main]
async fn main() {
    let chain = ChainBuilder::new()
        .link(AgentExecutor::new_claude_agent(TOOL_REGISTRY.deref()).await)
        .build();

    let output = chain
        .process("Is it hot outside?".to_string())
        .await
        .expect("Error processing chain");
    println!("{}", output);
}
