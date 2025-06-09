use rdev::Key;

/// Resolves a `rdev::Key` to a human-friendly label for rendering in the visualiser.
///
/// This function:
/// - Converts alphanumeric and numpad keys to their literal characters.
/// - Maps special keys (Shift, Ctrl, Alt, etc.) to icon-enhanced labels.
/// - Converts function and navigation keys to common names (e.g. F1–F12, Home, End).
/// - Handles known `Key::Unknown(code)` values used by media keys and other extended keys.
/// - Falls back to `Debug` output for any unhandled cases.
///
/// # Arguments
/// * `key` - A `rdev::Key` representing the physical key event.
///
/// # Returns
/// * A `String` containing the label to show in the UI.
pub fn resolve_physical_key(key: Key) -> String {
    match key {
        // Alphabetic keys
        Key::KeyA => "A".to_string(),
        Key::KeyB => "B".to_string(),
        Key::KeyC => "C".to_string(),
        Key::KeyD => "D".to_string(),
        Key::KeyE => "E".to_string(),
        Key::KeyF => "F".to_string(),
        Key::KeyG => "G".to_string(),
        Key::KeyH => "H".to_string(),
        Key::KeyI => "I".to_string(),
        Key::KeyJ => "J".to_string(),
        Key::KeyK => "K".to_string(),
        Key::KeyL => "L".to_string(),
        Key::KeyM => "M".to_string(),
        Key::KeyN => "N".to_string(),
        Key::KeyO => "O".to_string(),
        Key::KeyP => "P".to_string(),
        Key::KeyQ => "Q".to_string(),
        Key::KeyR => "R".to_string(),
        Key::KeyS => "S".to_string(),
        Key::KeyT => "T".to_string(),
        Key::KeyU => "U".to_string(),
        Key::KeyV => "V".to_string(),
        Key::KeyW => "W".to_string(),
        Key::KeyX => "X".to_string(),
        Key::KeyY => "Y".to_string(),
        Key::KeyZ => "Z".to_string(),

        // Number keys
        Key::Num1 => "1".to_string(),
        Key::Num2 => "2".to_string(),
        Key::Num3 => "3".to_string(),
        Key::Num4 => "4".to_string(),
        Key::Num5 => "5".to_string(),
        Key::Num6 => "6".to_string(),
        Key::Num7 => "7".to_string(),
        Key::Num8 => "8".to_string(),
        Key::Num9 => "9".to_string(),
        Key::Num0 => "0".to_string(),

        // Numpad keys
        Key::Kp0 => "0".to_string(),
        Key::Kp1 => "1".to_string(),
        Key::Kp2 => "2".to_string(),
        Key::Kp3 => "3".to_string(),
        Key::Kp4 => "4".to_string(),
        Key::Kp5 => "5".to_string(),
        Key::Kp6 => "6".to_string(),
        Key::Kp7 => "7".to_string(),
        Key::Kp8 => "8".to_string(),
        Key::Kp9 => "9".to_string(),
        Key::KpPlus => "+".to_string(),
        Key::KpDivide => "/".to_string(),
        Key::KpMinus => "-".to_string(),
        Key::KpMultiply => "*".to_string(),
        Key::KpReturn => "Enter".to_string(),
        Key::KpDelete => "Dot".to_string(),

        // Standard control keys
        Key::Return => "Enter".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::Escape => "Escape".to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::Space => "Space".to_string(),

        // Modifiers
        Key::ShiftLeft | Key::ShiftRight => "⇧ shift".to_string(),
        Key::ControlLeft | Key::ControlRight => "⌃ control".to_string(),
        Key::Alt | Key::AltGr => "⌥ alt".to_string(),
        Key::MetaLeft | Key::MetaRight => " Meta".to_string(),
        Key::CapsLock => "⇪ Caps".to_string(),

        // Function keys
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F5 => "F5".to_string(),
        Key::F6 => "F6".to_string(),
        Key::F7 => "F7".to_string(),
        Key::F8 => "F8".to_string(),
        Key::F9 => "F9".to_string(),
        Key::F10 => "F10".to_string(),
        Key::F11 => "F11".to_string(),
        Key::F12 => "F12".to_string(),

        // Punctuation and symbols
        Key::Minus => "-".to_string(),
        Key::Equal => "=".to_string(),
        Key::LeftBracket => "LeftBracket".to_string(),
        Key::RightBracket => "RightBracket".to_string(),
        Key::BackSlash => "BackSlash".to_string(),
        Key::SemiColon => "SemiColon".to_string(),
        Key::BackQuote => "BackQuote".to_string(),
        Key::Quote => "Quote".to_string(),
        Key::Comma => "Comma".to_string(),
        Key::Dot => "Period".to_string(),
        Key::Slash => "Slash".to_string(),

        // Navigation and editing
        Key::Insert => "Insert".to_string(),
        Key::Delete => "Delete".to_string(),
        Key::Home => "Home".to_string(),
        Key::End => "End".to_string(),
        Key::PageUp => "PageUp".to_string(),
        Key::PageDown => "PageDown".to_string(),
        Key::UpArrow => "UpArrow".to_string(),
        Key::DownArrow => "DownArrow".to_string(),
        Key::LeftArrow => "LeftArrow".to_string(),
        Key::RightArrow => "RightArrow".to_string(),

        // Known extended media and system keys (via `Unknown(n)`)
        Key::Unknown(12) => "".to_string(),
        Key::Unknown(172) => "󰋜 home".to_string(),
        Key::Unknown(173) => "󰖁 mute".to_string(),
        Key::Unknown(174) => "󰝞 vol-".to_string(),
        Key::Unknown(175) => "󰝝 vol+".to_string(),
        Key::Unknown(176) => "󰒭 next".to_string(),
        Key::Unknown(177) => "󰒮 prev".to_string(),
        Key::Unknown(178) => " stop".to_string(),
        Key::Unknown(179) => "󰐎 play".to_string(),
        Key::Unknown(180) => " mail".to_string(),
        Key::Unknown(181) => "󰝚 fn".to_string(),
        Key::Unknown(183) => "󰏋 App".to_string(),
        Key::Unknown(223) => "`".to_string(),

        // Fallback for unknown key codes
        Key::Unknown(code) => format!("󰘳 Unknown({})", code),

        // Catch-all fallback using Debug output
        k => format!("{:?}", k),
    }
}
