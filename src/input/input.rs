#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress(String),
    MouseClick(String),
}

// Delegate to platform-specific input backend
#[cfg(target_os = "windows")]
pub use crate::platform::windows::input::start_input_listener;

#[cfg(target_os = "linux")]
pub use crate::platform::linux::input::start_input_listener;

#[cfg(target_os = "macos")]
pub use crate::platform::macos::input::start_input_listener;
