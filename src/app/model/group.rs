use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Group {
    pub id: i32,
    pub admins: Vec<i32>,
    pub secret: String,
    pub confirmation: String,
}