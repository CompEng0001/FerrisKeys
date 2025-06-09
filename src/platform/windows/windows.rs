use rdev::Key;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::shared::minwindef::HKL;
use winapi::um::winuser::{GetKeyboardLayout, GetKeyboardState, MapVirtualKeyW, ToUnicodeEx};

/// Translates a given Windows virtual key code into its corresponding Unicode character(s),
/// considering the current keyboard layout and key state (e.g., Shift pressed).
///
/// Uses low-level Win32 APIs:
/// - `GetKeyboardLayout` to retrieve the active keyboard layout.
/// - `GetKeyboardState` to obtain modifier key states (e.g., Shift, Caps Lock).
/// - `MapVirtualKeyW` to convert the virtual key to a scan code.
/// - `ToUnicodeEx` to produce the Unicode output using the above state.
///
/// # Arguments
/// * `vk_code` - The virtual key code to translate (e.g., 0x41 for 'A').
///
/// # Returns
/// * `Some(String)` - If the virtual key translates into one or more Unicode characters.
/// * `None` - If translation fails or results in no output.
pub fn translate_key_win32(vk_code: u32) -> Option<String> {
    unsafe {
        let layout: HKL = GetKeyboardLayout(0);

        // Capture the full keyboard state (e.g., shift, ctrl)
        let mut key_state = [0u8; 256];
        if GetKeyboardState(key_state.as_mut_ptr()) == 0 {
            return None;
        }

        // Convert VK to scan code
        let scan_code = MapVirtualKeyW(vk_code, 0);

        // Buffer to hold UTF-16 characters from ToUnicodeEx
        let mut buffer = [0u16; 8];
        let result = ToUnicodeEx(
            vk_code,
            scan_code,
            key_state.as_ptr(),
            buffer.as_mut_ptr(),
            buffer.len() as i32,
            0,
            layout,
        );

        // Convert result to String if characters were returned
        if result > 0 {
            Some(
                OsString::from_wide(&buffer[..result as usize])
                    .to_string_lossy()
                    .into_owned(),
            )
        } else {
            None
        }
    }
}

/// Maps a high-level `rdev::Key` enum to its corresponding Windows virtual key code (VK code).
///
/// VK codes are platform-specific numeric constants defined by the Win32 API.
/// This mapping is essential for translating input events into printable characters.
///
/// # Arguments
/// * `key` - The `rdev::Key` to convert.
///
/// # Returns
/// * `Some(u32)` - The corresponding Windows VK code.
/// * `None` - If the key does not map to a known VK code (e.g., media keys, unsupported keys).
pub fn vk_code_from_key(key: Key) -> Option<u32> {
    use rdev::Key::*;
    match key {
        Num0 => Some(0x30),
        Num1 => Some(0x31),
        Num2 => Some(0x32),
        Num3 => Some(0x33),
        Num4 => Some(0x34),
        Num5 => Some(0x35),
        Num6 => Some(0x36),
        Num7 => Some(0x37),
        Num8 => Some(0x38),
        Num9 => Some(0x39),
        KeyA => Some(0x41),
        KeyB => Some(0x42),
        KeyC => Some(0x43),
        KeyD => Some(0x44),
        KeyE => Some(0x45),
        KeyF => Some(0x46),
        KeyG => Some(0x47),
        KeyH => Some(0x48),
        KeyI => Some(0x49),
        KeyJ => Some(0x4A),
        KeyK => Some(0x4B),
        KeyL => Some(0x4C),
        KeyM => Some(0x4D),
        KeyN => Some(0x4E),
        KeyO => Some(0x4F),
        KeyP => Some(0x50),
        KeyQ => Some(0x51),
        KeyR => Some(0x52),
        KeyS => Some(0x53),
        KeyT => Some(0x54),
        KeyU => Some(0x55),
        KeyV => Some(0x56),
        KeyW => Some(0x57),
        KeyX => Some(0x58),
        KeyY => Some(0x59),
        KeyZ => Some(0x5A),
        Space => Some(0x20),
        Minus => Some(0xBD),
        Equal => Some(0xBB),
        LeftBracket => Some(0xDB),
        RightBracket => Some(0xDD),
        BackSlash => Some(0xDC),
        SemiColon => Some(0xBA),
        Quote => Some(0xDE),
        Comma => Some(0xBC),
        Dot => Some(0xBE),
        Slash => Some(0xBF),
        _ => None,
    }
}
