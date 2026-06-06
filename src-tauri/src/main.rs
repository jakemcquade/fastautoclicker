#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod inputs;
mod modules;
mod commands;
mod state;

use tauri::{Emitter, Manager};
use std::sync::{Arc, Mutex};

static STORAGE: Mutex<String> = Mutex::new(String::new());

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
                            let state = app.state::<state::AppState>();
                            if modules::util::is_valid_hotkey(&json.hotkey) {
                                *state.hotkey.lock().unwrap() = json.hotkey;
                            }

                            modules::util::update_state(&state.mouse_button, json.mouse_button);
                            modules::util::update_state(&state.click_type, json.click_type);
                            modules::util::update_state(&state.repeat_count, json.repeat);
                            modules::util::update_state(
                                &state.interval,
                                modules::util::sanitize_interval(json.interval),
                            );
                        }
                        Err(err) => {
                            log::error!("Malformed settings.json: {}", err);
                        }
                    }
                }
                Err(_) => {}
            };

            let hotkey_arc = Arc::clone(&app.state::<state::AppState>().hotkey);
            let app_handle = app.handle().clone();
            modules::keylistener::spawn(hotkey_arc, move || {
                let state = app_handle.state::<state::AppState>();
                if *state.hotkey_recording.lock().unwrap() {
                    return;
                }

                state.toggle_click_loop(app_handle.clone());
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

            if let Err(e) = app.emit("single-instance", modules::util::Payload { args: argv, cwd }) {
                log::warn!("Failed to emit single-instance event: {e}");
            }
        }))
        .manage(state::AppState::default())
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
