mod app;

use tide::Request;
use tide::Result;
use app::app_state::AppState;
use chrono::Utc;
use app::model::callback::Callback;

const OK_RESPONSE: &str = "ok";
const CONFIRMATION_CODE: &str = "confirmation";

#[async_std::main]
async fn main() -> Result<()> {
    let state = match AppState::new("./data/settings.json") {
        Ok(state) => state,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    let address = state.setting.address.to_owned();
    let mut app = tide::with_state(state);
    app.at("/vk-groups-messages-mediator").post(handle);

    println!("server started at: {}", address);
    app.listen(address).await?;
    
    Ok(())
}

async fn handle(mut req: Request<AppState>) -> Result {
    let json = match req.body_string().await {
        Ok(json) => {
            println!("[{}] [Request]: {}", Utc::now(),  json);
            json
        },
        Err(err) => {
            println!("[{}] [Error]: {}", Utc::now(), err);
            return Ok(OK_RESPONSE.into());
        }        
    };

    let callback: Callback = match serde_json::from_str(&json) {
        Ok(callback) => callback,
        Err(err) => {
            println!("[{}] [Error]: {}", Utc::now(), err);
            return Ok(OK_RESPONSE.into());
        }
    };
    
    match callback.type_name.as_str() {
        CONFIRMATION_CODE => return handle_confirmation(req.state(), &callback).await,
        _ => {
            println!("[{}] [Error]: unknown type", Utc::now());
            return Ok(OK_RESPONSE.into());
        } 
    }
}

async fn handle_confirmation(state: &AppState, callback: &Callback) -> Result {
    match state.setting.get_confirmation(callback.group_id, &callback.secret) {
        Some(confirmation) => {
            println!("[{}] [Response]: {}", Utc::now(), confirmation);
            return Ok(confirmation.into());
        },
        None => {
            println!("[{}] [Error]: unknown group_id: {}", Utc::now(), callback.group_id);
            return Ok(OK_RESPONSE.into());
        }
    }
}