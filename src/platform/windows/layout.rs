use crate::input::layout::KeyboardLayout;
use winapi::shared::minwindef::HKL;
use winapi::um::processthreadsapi::GetCurrentThreadId;
use winapi::um::winuser::GetKeyboardLayout;

/// Detects the active keyboard layout on Windows using the thread's input language ID.
///
/// Internally uses the Windows API:
/// - `GetCurrentThreadId()` to get the current thread's ID.
/// - `GetKeyboardLayout()` to get the input locale identifier (HKL) for that thread.
///
/// The returned `HKL` contains the language ID in the high word. Known mappings:
/// - `0x0809` → `KeyboardLayout::UnitedKingdom`
/// - `0x0409` → `KeyboardLayout::UnitedStates`
///
/// Other layout IDs are returned as `KeyboardLayout::Other(layout_id)`.
///
/// # Returns
/// A `KeyboardLayout` enum variant corresponding to the current layout.
pub fn detect_layout() -> KeyboardLayout {
    unsafe {
        // Get the current thread ID
        let thread_id = GetCurrentThreadId();

        // Get the keyboard layout (HKL) for that thread
        let hkl: HKL = GetKeyboardLayout(thread_id);

        // Extract the high word (bits 16–31), which represents the layout ID
        let layout_id = ((hkl as usize >> 16) & 0xFFFF) as u16;

        // Match known layouts or return a generic fallback
        match layout_id {
            0x0809 => KeyboardLayout::UnitedKingdom, // English (UK)
            0x0409 => KeyboardLayout::UnitedStates,  // English (US)
            _ => KeyboardLayout::Other(layout_id),   // Other/unknown layout
        }
    }
}
