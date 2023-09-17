use super::app_settings::AppSettings;
use std::error::Error;

#[derive(Clone,Debug)]
pub struct AppState {
    pub setting: AppSettings
}

impl AppState {
    pub fn new(settings_path: &str) -> Result<AppState, Box<dyn Error>> {
        let settings = AppSettings::from_file(settings_path)?;
        let state = AppState {
            setting: settings,
        };

        Ok(state)
    }
}