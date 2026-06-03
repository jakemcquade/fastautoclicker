#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod inputs;
mod modules;
mod commands;

use tauri::{Emitter, Manager};
use std::sync::{mpsc, Arc, Mutex};

static STORAGE: Mutex<String> = Mutex::new(String::new());

pub struct AppState {
    status: Mutex<bool>,
    interval: Mutex<u64>,
    hotkey: Arc<Mutex<String>>,
    hotkey_recording: Mutex<bool>,
    click_type: Mutex<u8>,
    mouse_button: Mutex<u8>,
    click_location: Mutex<Option<(i32, i32)>>,
    stop_interval: Mutex<Option<mpsc::Sender<()>>>,
}

impl AppState {
    fn default() -> Self {
        Self {
            status: Mutex::new(false),
            interval: Mutex::new(100),
            hotkey: Arc::new(Mutex::new("F6".to_string())),
            hotkey_recording: Mutex::new(false),
            click_type: Mutex::new(0),
            mouse_button: Mutex::new(0),
            click_location: Mutex::new(None),
            stop_interval: Mutex::new(None),
        }
    }

    pub fn toggle_click_loop(&self) -> bool {
        let mut stop_interval = self.stop_interval.lock().unwrap();
        let mut status = self.status.lock().unwrap();
        let location = self.click_location.lock().unwrap().clone();
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
                    modules::util::send_click(mtype, location);
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
                    match serde_json::from_str::<modules::storage::ConfigStruct>(&contents) {
                        Ok(json) => {
                            let state = app.state::<AppState>();
                            if modules::util::is_valid_hotkey(&json.hotkey) {
                                *state.hotkey.lock().unwrap() = json.hotkey;
                            }

                            modules::util::update_state(&state.mouse_button, json.mouse_button);
                            modules::util::update_state(&state.click_type, json.click_type);
                            modules::util::update_state(&state.interval, json.interval);
                        }
                        Err(err) => {
                            log::error!("Malformed settings.json: {}", err);
                        }
                    }
                }
                Err(_) => {}
            };

            let hotkey_arc = Arc::clone(&app.state::<AppState>().hotkey);
            let app_handle = app.handle().clone();
            modules::keylistener::spawn(hotkey_arc, move || {
                let state = app_handle.state::<AppState>();
                if *state.hotkey_recording.lock().unwrap() {
                    return;
                }
                let result = state.toggle_click_loop();
                app_handle.emit("state", result).unwrap();
            });

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

            app.emit("single-instance",modules::util::Payload { args: argv, cwd }).unwrap();
        }))
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            modules::storage::get_settings,
            modules::storage::set_settings,
            commands::app_toggle,
            commands::app_stop,
            commands::set_state
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
}
