use futures_util::{SinkExt, StreamExt};
use serde_json::to_string;
use std::sync::Arc;
use tauri::Emitter;
use tauri::{Manager, Runtime, Window};
use tokio::sync::Mutex;
mod gamepad;
use gamepad::*;

// global state
struct AppState {
    gamepad: Arc<Mutex<GamepadState>>,
}



#[tauri::command]
async fn buttons_info<R: Runtime>(
    app_state: tauri::State<'_, AppState>,
    window: Window<R>, 
    btn: &str, 
    status: bool
) -> Result<(), String> {
    let mut gamepad = app_state.gamepad.lock().await;
    gamepad.update_button(btn, status);
    
    window.emit("send_to_server", gamepad.to_output_string())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn stick_info<R: Runtime>(
    app_state: tauri::State<'_, AppState>,
    window: Window<R>, 
    stick: &str, 
    x: i32, 
    y: i32
) -> Result<(), String> {
    let mut gamepad = app_state.gamepad.lock().await;
    gamepad.update_stick(stick, x, y);
    
    window.emit("send_to_server", gamepad.to_output_string())
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn trigger_info(
    app_state: tauri::State<'_, AppState>,
    window: Window<tauri::Wry>,
    trigger: String,
    value: u8,
) -> Result<(), String> {
    let mut gamepad = app_state.gamepad.lock().await;
    gamepad.update_trigger(&trigger, value);
    
    window.emit("send_to_server", gamepad.to_output_string())
        .map_err(|e| e.to_string())?;
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let gamepad_state = new_shared_state();
    let app_state = AppState {
        gamepad: gamepad_state,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_websocket::init())
        .manage(app_state) // sending app_state
        .invoke_handler(tauri::generate_handler![
            buttons_info,
            stick_info,
            trigger_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}