use super::data::Data;

#[derive(Debug, serde::Deserialize)]
pub struct Callback {
    #[serde(rename(deserialize = "type"))]
    pub type_name: String,
    
    #[serde(rename(deserialize = "secret"))]
    pub secret: String,
    
    #[serde(rename(deserialize = "group_id"))]
    pub group_id: i32,
    
    #[serde(rename(deserialize = "object"))]
    pub data: Data,
}