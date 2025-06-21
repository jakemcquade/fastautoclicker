#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod inputs;
mod modules;

use std::str::FromStr;
use std::sync::{ Mutex, mpsc };
use tauri::{ Emitter, Manager };
use tauri_plugin_global_shortcut::GlobalShortcutExt;

static STORAGE: Mutex<String> = Mutex::new(String::new());

pub struct AppState {
    status: Mutex<bool>,
    interval: Mutex<u64>,
    hotkey: Mutex<String>,
    click_type: Mutex<u8>,
    mouse_button: Mutex<u8>,
    stop_interval: Mutex<Option<mpsc::Sender<()>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            status: Mutex::new(false),
            interval: Mutex::new(100),
            stop_interval: Mutex::new(None),
            mouse_button: Mutex::new(0),
            click_type: Mutex::new(0),
            hotkey: Mutex::new("shift+tab".to_string())
        }
    }

    pub fn toggle_click_loop(&self) -> bool {
        let mut status = self.status.lock().unwrap();
        let mut stop_interval = self.stop_interval.lock().unwrap();
        *status = !*status;

        if *status {
            let (tx, rx) = mpsc::channel();
            let interval = std::time::Duration::from_millis(*self.interval.lock().unwrap());
            let mtype = crate::inputs::MouseButton::from(*self.mouse_button.lock().unwrap());
            let ctype = crate::inputs::ClickType::from(*self.click_type.lock().unwrap());
            
            std::thread::spawn(move || loop {
                let clicks = match ctype {
                    crate::inputs::ClickType::Single => 1,
                    crate::inputs::ClickType::Double => 2,
                };

                for _ in 0..clicks {
                    modules::util::send_click(mtype);
                }

                std::thread::sleep(interval);
                match rx.try_recv() {
                    Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
                    Err(std::sync::mpsc::TryRecvError::Empty) => {}
                }
            });

            *stop_interval = Some(tx);
        } else if let Some(tx) = stop_interval.take() {
            let _ = tx.send(());
        }

        log::info!("State: {}", if *status { "Enabled" } else { "Disabled" });
        *status
    }
}

// #[tauri::command]
// fn get_state(state: tauri::State<AppState>, name: String) -> Result<modules::util::Value, String> {
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
fn set_state(
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
                modules::util::update_state(&state.interval, val);
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
        "hotkey" => match value {
            modules::util::Value::String(val) => {
                if !modules::util::is_valid_hotkey(&val) {
                    return Err("Invalid hotkey: must start with a modifier key.".to_string());
                }

                modules::util::update_state(&state.hotkey, val.clone());

                let shortcut = tauri_plugin_global_shortcut::Shortcut::from_str(&val)
                    .map_err(|e| format!("Failed to parse shortcut: {:?}", e))?;
                app.global_shortcut()
                    .unregister_all()
                    .map_err(|e| format!("Failed to unregister shortcuts: {:?}", e))?;
                app.global_shortcut()
                    .register(shortcut)
                    .map_err(|e| format!("Failed to register shortcut: {:?}", e))?;

                app.emit("hotkey", val)
                    .map_err(|e| format!("Failed to emit hotkey event: {:?}", e))?;
                Ok(true)
            }
            _ => Err("Invalid value type for hotkey.".to_string()),
        },
        _ => Err(format!("Invalid state type: {}", name)),
    }
}

#[tauri::command]
fn app_toggle(state: tauri::State<AppState>, time: u64, mousebutton: u8, clicktype: u8) -> bool {
    *state.interval.lock().unwrap() = time;
    *state.mouse_button.lock().unwrap() = mousebutton;
    *state.click_type.lock().unwrap() = clicktype;

    state.toggle_click_loop()
}

#[tauri::command]
fn app_stop(state: tauri::State<AppState>) {
    let mut status = state.status.lock().unwrap();
    let mut stop_interval = state.stop_interval.lock().unwrap();
    if !*status || stop_interval.is_none() {
        log::warn!("No click loop is running to stop.");
        return;
    }

    log::info!("Stopping click loop...");
    *status = !*status;

    if let Some(tx) = stop_interval.take() {
        if let Err(e) = tx.send(()) {
            log::error!("Failed to send stop signal: {:?}", e.to_string());
            std::process::exit(1);
        }
    }

    log::info!("State: {}", if *status { "Enabled" } else { "Disabled" });
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            *STORAGE.lock().unwrap() = app
                .path()
                .app_config_dir()
                .unwrap_or_else(|_| std::path::PathBuf::new())
                .to_string_lossy()
                .to_string();

            modules::storage::init().unwrap();
            match std::fs::read_to_string(&format!("{}/settings.json", STORAGE.lock().unwrap())) {
                Ok(contents) => {
                    let contents_str: &str = contents.as_str();
                    let json: modules::storage::ConfigStruct =
                        serde_json::from_str(contents_str).expect("Malformed JSON.");
                    let state = app.state::<AppState>();

                    if modules::util::is_valid_hotkey(&json.hotkey) {
                        modules::util::update_state(&state.hotkey, json.hotkey);
                    }

                    modules::util::update_state(&state.mouse_button, json.mouse_button);
                    modules::util::update_state(&state.click_type, json.click_type);
                }
                Err(_) => {}
            };

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcuts([app.state::<AppState>().hotkey.lock().unwrap().as_str()])?
                    .with_handler(|app, shortcut, event| {
                        log::info!("Global Shortcut: {:?} - {:?}", shortcut, event);
                        if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                            let state = app.state::<AppState>();
                            let result = state.toggle_click_loop();
                            app.emit("state", result).unwrap();
                        }
                    })
                    .build(),
            )?;

            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir { file_name: None },
                ))
                .build(),
        )
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            if let Some(window) = app.get_webview_window("main") {
                if !window.is_visible().unwrap_or(false) {
                    let _ = window.show();
                }
            }

            app.emit("single-instance", modules::util::Payload { args: argv, cwd })
                .unwrap();
        }))
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            modules::storage::get_settings,
            modules::storage::set_settings,
            app_toggle,
            app_stop,
            // get_state,
            set_state
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
}
