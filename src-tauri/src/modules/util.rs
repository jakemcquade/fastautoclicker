use std::sync::Mutex;

use crate::inputs::{ MouseButton };

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
    U8(u8)
}

pub fn update_state<T>(lock: &Mutex<T>, value: T) -> bool {
    let mut data = lock.lock().unwrap();
    *data = value;
    true
}

pub const MIN_INTERVAL_MS: u64 = 1;
pub fn sanitize_interval(ms: u64) -> u64 {
    ms.max(MIN_INTERVAL_MS)
}

const VALID_MODIFIERS: &[&str] = &["Control", "Shift", "Alt", "Meta"];
const VALID_KEYS: &[&str] = &[
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
    "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
    "Tab", "Enter", "Escape", "Backspace", "CapsLock", "Space",
    "Delete", "Insert", "Home", "End", "PageUp", "PageDown",
    "ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight",
];

pub fn is_valid_hotkey(hotkey: &str) -> bool {
    let trimmed = hotkey.trim();
    if trimmed.is_empty() {
        return false;
    }

    let parts: Vec<&str> = trimmed.split('+').collect();
    let (trigger, modifiers) = parts.split_last().unwrap();

    let trigger_ok = VALID_KEYS.iter().any(|k| k.eq_ignore_ascii_case(trigger));
    let modifiers_ok = modifiers.iter().all(|m| VALID_MODIFIERS.contains(m));

    trigger_ok && modifiers_ok
}

pub fn send_click(mtype: MouseButton, position: Option<(i32, i32)>) {
    #[cfg(target_os = "windows")]
    {
        if let Some((x, y)) = position {
            crate::inputs::winput::set_position(x, y);
        }

        crate::inputs::winput::send(mtype);
    }

    #[cfg(target_os = "macos")]
    {
        crate::inputs::minput::send(mtype, position);
    }

    #[cfg(target_os = "linux")]
    {
        if let Err(err) = crate::inputs::linput::send(mtype, position) {
            log::warn!("Linux input failed: {}", err);
        }
    }
}
