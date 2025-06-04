use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use rdev::Key;
use winapi::shared::minwindef::HKL;
use winapi::um::winuser::{
    GetKeyboardLayout, 
    GetKeyboardState, 
    MapVirtualKeyW, 
    ToUnicodeEx
};


pub fn translate_key_win32(vk_code: u32) -> Option<String> {
    unsafe {
        let layout: HKL = GetKeyboardLayout(0);

        // Get keyboard state
        let mut key_state = [0u8; 256];
        if GetKeyboardState(key_state.as_mut_ptr()) == 0 {
            return None;
        }

        // Translate virtual key to scan code
        let scan_code = MapVirtualKeyW(vk_code, 0);

        // Convert to Unicode string
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

        if result > 0 {
            Some(OsString::from_wide(&buffer[..result as usize]).to_string_lossy().into_owned())
        } else {
            None
        }
    }
}

pub fn vk_code_from_key(key: Key) -> Option<u32> {
    use rdev::Key::*;
    match key {
        Num0 => Some(0x30), Num1 => Some(0x31), Num2 => Some(0x32), Num3 => Some(0x33),
        Num4 => Some(0x34), Num5 => Some(0x35), Num6 => Some(0x36), Num7 => Some(0x37),
        Num8 => Some(0x38), Num9 => Some(0x39),
        KeyA => Some(0x41), KeyB => Some(0x42), KeyC => Some(0x43), KeyD => Some(0x44),
        KeyE => Some(0x45), KeyF => Some(0x46), KeyG => Some(0x47), KeyH => Some(0x48),
        KeyI => Some(0x49), KeyJ => Some(0x4A), KeyK => Some(0x4B), KeyL => Some(0x4C),
        KeyM => Some(0x4D), KeyN => Some(0x4E), KeyO => Some(0x4F), KeyP => Some(0x50),
        KeyQ => Some(0x51), KeyR => Some(0x52), KeyS => Some(0x53), KeyT => Some(0x54),
        KeyU => Some(0x55), KeyV => Some(0x56), KeyW => Some(0x57), KeyX => Some(0x58),
        KeyY => Some(0x59), KeyZ => Some(0x5A),
        Space => Some(0x20),
        Minus => Some(0xBD), Equal => Some(0xBB),
        LeftBracket => Some(0xDB), RightBracket => Some(0xDD),
        BackSlash => Some(0xDC), SemiColon => Some(0xBA),
        Quote => Some(0xDE), Comma => Some(0xBC), Dot => Some(0xBE),
        Slash => Some(0xBF),
        _ => None,
    }
}