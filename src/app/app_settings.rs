use super::model::group::Group;
use std::{fs, error::Error};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppSettings {
    //TODO: move to command line parameters
    pub address: String,
    pub groups: Vec<Group>,    
}

impl AppSettings {
    pub fn from_file(file_path: &str) -> Result<AppSettings, Box<dyn Error>> {
        let json = fs::read_to_string(file_path)?;
        let settings: AppSettings = serde_json::from_str(&json)?;
        Ok(settings)
    }

    pub fn get_confirmation(self: &AppSettings, id: i32, secret: &str) -> Option<&str> {
        for group in &self.groups {
            if group.id == id && group.secret == secret {
                return Some(group.confirmation.as_str())
            }
        }

        None
    }
}