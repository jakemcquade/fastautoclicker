use std::sync::Mutex;

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub args: Vec<String>,
    pub cwd: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Value {
    String(String),
    Bool(bool),
    U64(u64),
}

pub fn update_state<T>(lock: &Mutex<T>, value: T) -> bool {
    let mut data = lock.lock().unwrap();
    *data = value;
    true
}

pub fn is_valid_hotkey(hotkey: &str) -> bool {
    let modifiers = ["Ctrl", "Alt", "Shift", "Cmd"];
    let parts: Vec<&str> = hotkey.split('+').collect();
    if parts.is_empty() { return false; }

    modifiers.contains(&parts[0])
}

pub fn send_click(mtype: String) {
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