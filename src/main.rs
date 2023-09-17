mod app;

use tide::Request;
use tide::Result;
use app::app_state::AppState;
use chrono::Utc;
use app::model::callback::Callback;

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
            return Ok("".into());
        }        
    };

    let callback: Callback = match serde_json::from_str(&json) {
        Ok(callback) => callback,
        Err(err) => {
            println!("[{}] [Error]: {}", Utc::now(), err);
            return Ok("".into());
        }
    };
    
    match callback.type_name.as_str() {
        "confirmation" => return handle_confirmation(req.state(), &callback).await,
        _ => {
            println!("[{}] [Error]: unknown type", Utc::now());
            return Ok("".into());
        } 
    }
}

async fn handle_confirmation(state: &AppState, callback: &Callback) -> Result {
    match state.setting.get_confirmation(callback.group_id) {
        Some(confirmation) => {
            println!("[{}] [Response]: {}", Utc::now(), confirmation);
            return Ok(confirmation.into());
        },
        None => {
            println!("[{}] [Error]: unknown group_id: {}", Utc::now(), callback.group_id);
            return Ok("".into());
        }
    }
}