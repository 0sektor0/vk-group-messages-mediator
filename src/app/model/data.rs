use serde_json::Value;

#[derive(Debug, serde::Deserialize)]
pub struct Data {
    pub message: Option<Value>,
}