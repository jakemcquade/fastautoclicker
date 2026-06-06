use crate::STORAGE;

use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(serde::Deserialize)]
#[serde(default)]
pub struct ConfigStruct {
    pub mouse_button: u8,
    pub click_type: u8,
    pub hotkey: String,
    pub interval: u64,
    pub repeat: u64
}

impl Default for ConfigStruct {
    fn default() -> Self {
        Self {
            hotkey: "F6".to_string(),
            mouse_button: 0,
            click_type: 0,
            interval: 100,
            repeat: 0
        }
    }
}

#[tauri::command]
pub fn get_settings() -> Result<String, String> {
    let config_dir = STORAGE.lock().map_err(|e| e.to_string())?;
    let path = Path::new(config_dir.as_str()).join("settings.json");
    fs::read_to_string(&path).map_err(|e| format!("Failed to read settings: {e}"))
}

#[tauri::command]
pub fn set_settings(file_content: String) -> Result<(), String> {
    serde_json::from_str::<ConfigStruct>(&file_content)
        .map_err(|e| format!("Invalid settings JSON: {e}"))?;

    let config_dir = STORAGE.lock().map_err(|e| e.to_string())?;
    let path = Path::new(config_dir.as_str()).join("settings.json");
    fs::write(&path, &file_content).map_err(|e| format!("Failed to write settings: {e}"))
}

fn create_dir_if_not_exists(path: &str) {
    if !Path::new(path).exists() {
        fs::create_dir_all(path).expect("Failed to create dir.");
    }
}

fn extract_keys(json: &Value) -> Vec<String> {
    match json {
        Value::Object(obj) => obj.keys().map(|s| s.to_string()).collect(),
        _ => Vec::new(),
    }
}

pub fn init() -> Result<(), std::io::Error> {
    let config_dir = STORAGE.lock().unwrap();
    create_dir_if_not_exists(&config_dir);

    let default_settings = r#"{
        "hotkey": "F6",
        "mouse_button": 0,
        "click_type": 0,
        "interval": 100,
        "repeat": 0
    }"#;

    let settings = &format!("{}/settings.json", &config_dir);
    if !Path::new(settings).exists() {
        writeln!(fs::File::create(settings)?, "{}", default_settings)?;
    }

    let expected_config: Value = serde_json::from_str(default_settings)?;
    let actual_config: Value = serde_json::from_str(&fs::read_to_string(&settings).unwrap())?;

    let expected_keys = extract_keys(&expected_config);
    let actual_keys = extract_keys(&actual_config);
    let missing_keys: Vec<&String> = expected_keys
        .iter()
        .filter(|key| !actual_keys.contains(*key))
        .collect();

    if !missing_keys.is_empty() {
        let mut config = actual_config;
        if let Value::Object(ref mut obj) = config {
            for key in missing_keys {
                obj.insert(key.clone(), expected_config[key].clone());
            }
        }

        fs::write(settings, config.to_string())?;
    }

    Ok(())
}