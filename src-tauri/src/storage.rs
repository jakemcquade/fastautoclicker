use crate::STORAGE;

use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;

#[tauri::command]
pub fn get_settings() -> Result<String, ()> {
    let config_dir = STORAGE.lock().unwrap();
    Ok(fs::read_to_string(&format!("{}/settings.json", &config_dir)).unwrap())
}

#[tauri::command]
pub async fn set_settings(file_content: String) {
    let config_dir = STORAGE.lock().unwrap();
    fs::write(&format!("{}/settings.json", &config_dir), file_content)
        .expect("Unable to write file.");
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
        "hotkey": "Shift+Q",
        "mouse_button": "left",
        "click_type": "single"
    }"#;

    let settings = &format!("{}/settings.json", &config_dir);
    if !Path::new(settings).exists() {
        writeln!(fs::File::create(settings)?, "{}", default_settings)?;
    }

    let expected_config: Value = serde_json::from_str(default_settings)?;
    let actual_config: Value = serde_json::from_str(&fs::read_to_string(&settings).unwrap())?;

    let expected_keys = extract_keys(&expected_config);
    let actual_keys = extract_keys(&actual_config);
    if actual_keys != expected_keys {
        let missing_keys: Vec<&String> = expected_keys.iter().filter(|key| !actual_keys.contains(*key)).collect();
        for key in &missing_keys {
            let mut config: Value = serde_json::from_str(&fs::read_to_string(&settings).unwrap())?;
            if let Some(value) = expected_config[key].as_str() {
                if let Value::Object(ref mut obj) = config {
                    obj.insert(key.to_string(), value.into());
                }

                fs::write(&default_settings, config.to_string())?;
            } else if let Some(value) = expected_config[key].as_bool() {
                if let Value::Object(ref mut obj) = config {
                    obj.insert(key.to_string(), value.into());
                }

                fs::write(&default_settings, config.to_string())?;
            }
        }
    }

    Ok(())
}