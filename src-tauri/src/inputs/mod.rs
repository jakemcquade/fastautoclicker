#[derive(Copy, Clone)]
pub enum Button { Left, Right, Middle }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Action { Press, Release }

#[cfg(target_os = "windows")]
pub mod winput;

#[cfg(target_os = "macos")]
pub mod minput;