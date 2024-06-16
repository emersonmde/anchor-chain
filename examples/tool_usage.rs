use anchor_chain::ToolRegistry;
use anchor_chain_macros::tool;
use once_cell::sync::Lazy;
use serde_json::Value;
use tokio::sync::RwLock;

static TOOL_REGISTRY: Lazy<RwLock<ToolRegistry>> = Lazy::new(|| RwLock::new(ToolRegistry::new()));

#[tool(TOOL_REGISTRY)]
/// This is a foo function
///
/// This is another line
fn foo(one: String, two: String) {
    println!("Foobar {one} {two}")
}

#[tool(TOOL_REGISTRY)]
/// This is a bar function
fn bar(x: i32, y: i32) -> i32 {
    x + y
}

#[tokio::main]
async fn main() {
    let params = serde_json::json!({"one": "baz", "two": "bam"});
    TOOL_REGISTRY
        .read()
        .await
        .execute_tool("foo", params)
        .unwrap();
    println!(
        "Foo schema: {:?}",
        TOOL_REGISTRY.read().await.get_schema("foo").unwrap()
    );

    let params = serde_json::json!({"x": 1, "y": 2});
    let result = TOOL_REGISTRY
        .read()
        .await
        .execute_tool("bar", params)
        .unwrap();
    println!("Bar result: {}", result);
    println!(
        "Bar schema: {:?}",
        TOOL_REGISTRY.read().await.get_schema("bar").unwrap()
    );
}
