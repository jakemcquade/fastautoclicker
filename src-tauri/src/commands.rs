use tauri::Emitter;

use crate::state::AppState;
use crate::modules;

#[tauri::command]
pub fn app_toggle(app: tauri::AppHandle, state: tauri::State<AppState>) -> bool {
    state.toggle_click_loop(app)
}

#[tauri::command]
pub fn app_stop(state: tauri::State<AppState>) {
    let mut stop_interval = state.stop_interval.lock().unwrap();
    let mut status = state.status.lock().unwrap();
    if !*status || stop_interval.is_none() {
        log::warn!("No click loop is running to stop.");
        return;
    }

    log::info!("Stopping click loop...");
    *status = !*status;

    if let Some(tx) = stop_interval.take() {
        if tx.send(()).is_err() {
            log::warn!("Click loop already stopped; stop signal had no receiver.");
        }
    }

    log::info!("State: {}", if *status { "Enabled" } else { "Disabled" });
}

// #[tauri::command]
// pub fn get_state(state: tauri::State<AppState>, name: String) -> Result<modules::util::Value, String> {
//     log::info!("GET state: {:?}", name);
//     match name.as_str() {
//         "status" => Ok(modules::util::Value::Bool(*state.status.lock().unwrap())),
//         "interval" => Ok(modules::util::Value::U64(*state.interval.lock().unwrap())),
//         "mouse_button" => Ok(modules::util::Value::U8(*state.mouse_button.lock().unwrap())),
//         "click_type" => Ok(modules::util::Value::U8(*state.click_type.lock().unwrap())),
//         "hotkey" => Ok(modules::util::Value::String(state.hotkey.lock().unwrap().clone())),
//         _ => return Err(format!("Invalid state type: {}", name)),
//     }
// }

#[tauri::command]
pub fn set_state(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    name: String,
    value: modules::util::Value,
) -> Result<bool, String> {
    log::info!("Set state: {:?} = {:?}", name, value);
    match name.as_str() {
        "status" => match value {
            modules::util::Value::Bool(val) => {
                modules::util::update_state(&state.status, val);
                Ok(true)
            }
            _ => Err("Invalid value type for status.".to_string()),
        },
        "interval" => match value {
            modules::util::Value::U64(val) => {
                if val < modules::util::MIN_INTERVAL_MS {
                    return Err(format!("Interval must be at least {}ms.", modules::util::MIN_INTERVAL_MS));
                }

                modules::util::update_state(&state.interval, modules::util::sanitize_interval(val));
                Ok(true)
            }
            _ => Err("Invalid value type for interval.".to_string()),
        },
        "mouse_button" => match value {
            modules::util::Value::U8(val) => {
                modules::util::update_state(&state.mouse_button, val);
                Ok(true)
            }
            _ => Err("Invalid value type for mouse_button.".to_string()),
        },
        "click_type" => match value {
            modules::util::Value::U8(val) => {
                modules::util::update_state(&state.click_type, val);
                Ok(true)
            }
            _ => Err("Invalid value type for click_type.".to_string()),
        },
        "repeat" => match value {
            modules::util::Value::U64(val) => {
                modules::util::update_state(&state.repeat_count, val);
                Ok(true)
            }
            _ => Err("Invalid value type for repeat.".to_string()),
        },
        "click_location" => match value {
            modules::util::Value::String(val) => {
                let coords: Vec<&str> = val.split(',').collect();
                if coords.len() != 2 {
                    return Err("Invalid click location format. Use 'x,y'.".to_string());
                }

                let x = coords[0].parse::<i32>().map_err(|_| "Invalid x coordinate.".to_string())?;
                let y = coords[1].parse::<i32>().map_err(|_| "Invalid y coordinate.".to_string())?;
                modules::util::update_state(&state.click_location, Some((x, y)));
                Ok(true)
            }
            _ => Err("Invalid value type for click_location.".to_string()),
        },
        "hotkey" => match value {
            modules::util::Value::String(val) => {
                if !modules::util::is_valid_hotkey(&val) {
                    return Err("Invalid hotkey.".to_string());
                }

                *state.hotkey.lock().unwrap() = val.clone();

                app.emit("hotkey", val)
                    .map_err(|e| format!("Failed to emit hotkey event: {:?}", e))?;
                Ok(true)
            }
            _ => Err("Invalid value type for hotkey.".to_string()),
        },
        "hotkey_recording" => match value {
            modules::util::Value::Bool(val) => {
                modules::util::update_state(&state.hotkey_recording, val);
                Ok(true)
            }
            _ => Err("Invalid value type for hotkey_recording.".to_string()),
        },
        _ => Err(format!("Invalid state type: {}", name)),
    }
}
