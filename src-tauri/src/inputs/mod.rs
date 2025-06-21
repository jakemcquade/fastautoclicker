#[derive(Copy, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle
}

impl From<u8> for MouseButton {
    fn from(value: u8) -> Self {
        match value {
            1 => MouseButton::Right,
            2 => MouseButton::Middle,
            _ => MouseButton::Left
        }
    }
}

#[derive(Copy, Clone)]
pub enum ClickType {
    Single,
    Double
}

impl From<u8> for ClickType {
    fn from(val: u8) -> Self {
        match val {
            1 => ClickType::Double,
            _ => ClickType::Single,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Action { Press, Release }
#[cfg(target_os = "windows")]
pub mod winput;

#[cfg(target_os = "macos")]
pub mod minput;