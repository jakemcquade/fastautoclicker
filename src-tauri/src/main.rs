#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod storage;
mod inputs;

use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tauri::{Emitter, Manager};
use std::sync::{Arc, Mutex};
use log::{error, info};
use std::str::FromStr;
use std::sync::mpsc;

static STORAGE: Mutex<String> = Mutex::new(String::new());

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

pub struct AppState {
    status: Arc<Mutex<bool>>,
    interval: Arc<Mutex<u64>>,
    hotkey: Arc<Mutex<String>>,
    click_type: Arc<Mutex<String>>,
    mouse_button: Arc<Mutex<String>>,
    stop_interval: Arc<Mutex<Option<mpsc::Sender<()>>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(false)),
            interval: Arc::new(Mutex::new(100)),
            stop_interval: Arc::new(Mutex::new(None)),
            mouse_button: Arc::new(Mutex::new("left".to_string())),
            click_type: Arc::new(Mutex::new("single".to_string())),
            hotkey: Arc::new(Mutex::new("shift+q".to_string())),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum Value {
    String(String),
    Bool(bool),
    U64(u64),
}

#[tauri::command]
fn get_state(state: tauri::State<AppState>, name: String) -> Value {
    info!("GET state: {:?}", name);
    match name.as_str() {
        "status" => Value::Bool(*state.status.lock().unwrap()),
        "interval" => Value::U64(*state.interval.lock().unwrap()),
        "mouse_button" => Value::String(state.mouse_button.lock().unwrap().clone()),
        "click_type" => Value::String(state.click_type.lock().unwrap().clone()),
        "hotkey" => Value::String(state.hotkey.lock().unwrap().clone()),
        _ => panic!("Invalid statetype \"{:?}\".", name),
    }
}

#[tauri::command]
fn set_state(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    name: String,
    value: Value,
) -> bool {
    info!("Set state: {:?} = {:?}", name, value);
    match name.as_str() {
        "status" => match value {
            Value::Bool(val) => {
                let mut status = state.status.lock().unwrap();
                *status = val;

                return true;
            }
            _ => panic!("Invalid value type."),
        },
        "interval" => match value {
            Value::U64(val) => {
                let mut data = state.interval.lock().unwrap();
                *data = val;

                return true;
            }
            _ => panic!("Invalid value type."),
        },
        "mouse_button" => match value {
            Value::String(val) => {
                let mut data = state.mouse_button.lock().unwrap();
                *data = val;

                return true;
            }
            _ => panic!("Invalid value type."),
        },
        "click_type" => match value {
            Value::String(val) => {
                let mut data = state.click_type.lock().unwrap();
                *data = val;

                return true;
            }
            _ => panic!("Invalid value type."),
        },
        "hotkey" => match value {
            Value::String(val) => {
                let mut data = state.hotkey.lock().unwrap();
                *data = val.clone();

                let shortcut = Shortcut::from_str(&val).unwrap();
                app.global_shortcut().unregister_all().unwrap();
                app.global_shortcut().register(shortcut).unwrap();

                let _ = app.emit("hotkey", val).unwrap();
                return true;
            }
            _ => panic!("Invalid value type."),
        },
        _ => panic!("Invalid statetype"),
    }
}

#[tauri::command]
fn app_toggle(
    state: tauri::State<AppState>,
    time: u64,
    mousebutton: String,
    clicktype: String,
) -> bool {
    let mut status = state.status.lock().unwrap();
    let mut interval = state.interval.lock().unwrap();
    let mut mouse_button = state.mouse_button.lock().unwrap();
    let mut click_type = state.click_type.lock().unwrap();
    let mut stop_interval = state.stop_interval.lock().unwrap();
    *mouse_button = mousebutton;
    *click_type = clicktype;
    *status = !*status;
    *interval = time;

    if *status {
        let (tx, rx) = mpsc::channel();
        let interval = std::time::Duration::from_millis(time);
        let mtype = mouse_button.clone().to_owned();
        let ctype = click_type.clone().to_owned();
        
        info!("Spawning Click Thread... {:?} - {:?} - {:?}", mtype, ctype, interval);
        std::thread::spawn(move || loop {
            for n in 1..3 {
                if ctype == "single" && n == 2 { break; };
                send_click(mtype.clone());
            }

            std::thread::sleep(interval);
            match rx.try_recv() {
                Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    break;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
            }
        });

        *stop_interval = Some(tx);
    } else {
        if let Some(tx) = stop_interval.take() {
            if let Err(e) = tx.send(()) {
                error!("Failed to send stop signal: {:?}", e.to_string());
                std::process::exit(1);
            }
        }
    }

    info!("State: {}", *status);
    return *status;
}

fn send_click(mtype: String) {
    #[cfg(target_os = "windows")] {
        crate::inputs::winput::send(match mtype.as_str() {
            "left" => crate::inputs::Button::Left,
            "middle" => crate::inputs::Button::Middle,
            "right" => crate::inputs::Button::Right,
            _ => crate::inputs::Button::Left
        });
    }

    #[cfg(target_os = "macos")] {
        crate::inputs::minput::send(match mtype.as_str() {
            "left" => crate::inputs::Button::Left,
            "middle" => crate::inputs::Button::Middle,
            "right" => crate::inputs::Button::Right,
            _ => crate::inputs::Button::Left
        });
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            *STORAGE.lock().unwrap() = app
                .path()
                .app_config_dir()
                .unwrap_or(std::path::PathBuf::new())
                .to_string_lossy()
                .to_string();

            #[derive(serde::Deserialize)]
            struct ConfigStruct {
                mouse_button: String,
                click_type: String,
                hotkey: String,
            }

            let _ = storage::init();
            match std::fs::read_to_string(&format!("{}/settings.json", STORAGE.lock().unwrap())) {
                Ok(contents) => {
                    let contents_str: &str = contents.as_str();
                    let json: ConfigStruct =
                        serde_json::from_str(contents_str).expect("Malformed JSON.");
                    let state = app.state::<AppState>();

                    let mut mouse_button = state.mouse_button.lock().unwrap();
                    *mouse_button = json.mouse_button;

                    let mut click_type = state.click_type.lock().unwrap();
                    *click_type = json.click_type;

                    let mut hotkey = state.hotkey.lock().unwrap();
                    *hotkey = json.hotkey;
                }
                Err(_) => {}
            };

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcuts([app.state::<AppState>().hotkey.lock().unwrap().as_str()])?
                    .with_handler(|app, shortcut, event| {
                        info!("Global Shortcut: {:?} - {:?}", shortcut, event);
                        if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                            let state = app.state::<AppState>();
                            let mut status = state.status.lock().unwrap();
                            let mut stop_interval = state.stop_interval.lock().unwrap();
                            let interval = state.interval.lock().unwrap();
                            let mouse_button = state.mouse_button.lock().unwrap();
                            let click_type = state.click_type.lock().unwrap();
                            *status = !*status;

                            if *status {
                                let (tx, rx) = mpsc::channel();
                                let interval = std::time::Duration::from_millis(*interval);
                                let mtype = mouse_button.clone().to_owned();
                                let ctype = click_type.clone().to_owned();

                                info!("Spawning Click Thread... {:?} - {:?} - {:?}", mtype, ctype, interval);
                                std::thread::spawn(move || loop {
                                    for n in 1..3 {
                                        if ctype == "single" && n == 2 { break; };
                                        send_click(mtype.clone());
                                    }

                                    std::thread::sleep(interval);
                                    match rx.try_recv() {
                                        Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                                            break;
                                        }
                                        Err(std::sync::mpsc::TryRecvError::Empty) => {}
                                    }
                                });

                                *stop_interval = Some(tx);
                            } else {
                                if let Some(tx) = stop_interval.take() {
                                    info!("Stopping click loop...");
                                    if let Err(e) = tx.send(()) {
                                        error!("Failed to send stop signal: {:?}", e.to_string());
                                        std::process::exit(1);
                                    }
                                }
                            }

                            app.emit("state", *status).unwrap();
                            info!("State: {}", *status);
                        }
                    })
                    .build(),
            )?;

            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir { file_name: None }))
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout))
                .format(move |out, message, record| {
                    out.finish(format_args!("{}[{}] {}", chrono::Local::now().format(format!("[%Y-%m-%d][%H:%M:%S]").as_str()), record.level(), message))
                })
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            let window = app.get_webview_window("main").unwrap();
            if !window.is_visible().unwrap() { window.show().unwrap(); };

            app.emit("single-instance", Payload { args: argv, cwd }).unwrap();
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            storage::get_settings,
            storage::set_settings,
            app_toggle,
            get_state,
            set_state
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application")
}