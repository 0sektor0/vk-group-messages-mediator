#[derive(Debug, Clone, serde::Deserialize)]
pub struct Group {
    pub id: i32,
    pub admins: Vec<i32>,
    pub secret: String,
    pub confirmation: String,
}