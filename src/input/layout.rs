#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardLayout {
    UnitedStates,
    UnitedKingdom,
    Other(u16),
}

// Platform-specific layout detection
/*#[cfg(target_os = "windows")]
pub use crate::platform::windows::layout::detect_layout;

#[cfg(target_os = "linux")]
pub use crate::platform::linux::layout::detect_layout;

#[cfg(target_os = "macos")]
pub use crate::platform::macos::layout::detect_layout;
*/
