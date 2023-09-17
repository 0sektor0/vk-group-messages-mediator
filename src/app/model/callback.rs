use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Callback {
    #[serde(rename(deserialize = "type"))]
    pub type_name: String,
    pub secret: String,
    pub group_id: i32,
}