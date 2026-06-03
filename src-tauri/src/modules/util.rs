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

pub fn is_valid_hotkey(hotkey: &str) -> bool {
    !hotkey.trim().is_empty()
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
        crate::inputs::minput::send(mtype);
    }

    #[cfg(target_os = "linux")]
    {
        if let Err(err) = crate::inputs::linput::send(mtype, position) {
            log::warn!("Linux input failed: {}", err);
        }
    }
}
