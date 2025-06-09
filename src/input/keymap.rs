use crate::input::layout::KeyboardLayout;
use rdev::Key;

/// Categorizes keys into visual styling groups.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyCategory {
    Escape,
    Normal,
    Numeric,
    Modifier,
    Editor,
    Navigation,
    Scrollable,
    Space,
    Symbol,
    Unknown,
    Function,
    AltFunction,
    Mouse,
}

/// Determines the category of a key based on its label.
///
/// Categories are used for visual styling, grouping, and filtering.
///
/// # Arguments
/// * `key` - A normalized key label.
///
/// # Returns
/// * `KeyCategory` representing how the key should be classified.
pub fn category_for_key(key: &str) -> KeyCategory {
    match key.to_ascii_lowercase().as_str() {
        "󰍽" | "left" | "right" | "middle" => KeyCategory::Mouse,
        "meta" | "esc" | "escape" | "\u{f0206} esc" => KeyCategory::Escape,
        "ctrl" | "control" | "⌃ control" | "shift" | "⇧ shift" | "alt" | "⌥ alt" | "tab"
        | "num" | "numlock" | "caps" => KeyCategory::Modifier,

        "󰹑" | "ps" | "backspace" | "delete" | "del" | "back" | "ins" | "insert" => {
            KeyCategory::Editor
        }

        "↑" | "↓" | "←" | "→" => KeyCategory::Navigation,

        "home" | "end" | "pageup" | "pagedown" | "pgup" | "pgdn" | "scroll" | "scrollock" => {
            KeyCategory::Scrollable
        }

        "space" | "󱁐 space" => KeyCategory::Space,

        // Common symbols used in programming and input
        "{" | "}" | "<" | ">" | "|" | "£" | "$" | "%" | "^" | "&" | "_" | "¬" | "#" | "`" | "("
        | ")" | "@" | "+" | "-" | "=" | "*" | "\\" | "/" | "," | "." | ";" | ":" | "!" | "'"
        | "[" | "]" | "?" | "~" | "\"" => KeyCategory::Symbol,

        // Function keys like F1–F24
        k if k.starts_with('f')
            && k[1..]
                .parse::<u8>()
                .map_or(false, |n| (1..=24).contains(&n)) =>
        {
            KeyCategory::Function
        }

        // Media and system keys
        k if k.contains("vol")
            || k.contains("mute")
            || k.contains("play")
            || k.contains("prev")
            || k.contains("next")
            || k.contains("stop")
            || k.contains("fn")
            || k.contains("web")
            || k.contains("mail")
            || k.contains("app")
            || k.contains("home") =>
        {
            KeyCategory::AltFunction
        }

        // Digits only
        k if k.chars().all(|c| c.is_ascii_digit()) => KeyCategory::Numeric,

        // Alphabetic only
        k if k.chars().all(|c| c.is_ascii_alphabetic()) => KeyCategory::Normal,

        _ => KeyCategory::Unknown,
    }
}

/// Converts raw mouse button identifiers into standardized labels with icons.
///
/// # Arguments
/// * `raw` - A `&str` representing the raw rdev mouse button label.
///
/// # Returns
/// * A standardized label with an icon, or the original string if unrecognized.
pub fn normalize_mouse_label(raw: &str) -> &str {
    match raw {
        "MouseLeft" => "󰍽 left",
        "MouseRight" => "󰍽 right ",
        "MouseMiddle" => "󰍽 middle",
        _ => raw,
    }
}

/// Maps internal rdev key names to user-friendly or symbolic labels for display.
///
/// # Arguments
/// * `raw` - The raw key name as a `&str`, usually from `format!("{:?}", Key)`.
///
/// # Returns
/// * A normalized, possibly symbolic label.
pub fn normalize_key_label(raw: &str) -> &str {
    match raw {
        "Comma" => ",",
        "Period" | "Dot" => ".",
        "SemiColon" => ";",
        "Colon" => ":",
        "BackQuote" => "'",
        "Apostrophe" => "'",
        "Minus" => "-",
        "Equal" => "=",
        "Slash" => "/",
        "BackSlash" | "IntlBackslash" => "\\",
        "Grave" => "`",
        "LeftBracket" => "[",
        "RightBracket" => "]",
        "Quote" => "#",
        "Space" => "󱁐 space",
        "Return" | "Enter" => "󰌑 enter",
        "Tab" => "Tab",
        "Backspace" => "󰭜 back",
        "Escape" => "󰈆 esc",
        "ShiftLeft" | "ShiftRight" => "⇧ shift",
        "ControlLeft" | "ControlRight" => "⌃ control",
        "Alt" | "AltGr" => "⌥ alt",
        "Meta" => "",
        "UpArrow" => "↑",
        "DownArrow" => "↓",
        "LeftArrow" => "←",
        "RightArrow" => "→",
        "Delete" => "⌦ del",
        "Insert" => " ins",
        "Home" => " home",
        "End" => " end",
        "PageUp" => "󰞕 pgup",
        "PageDown" => "󰞒 pgdn",
        "NumLock" => "󰍁 numlock",
        "ScrollLock" => "󰹹 scroll",
        "CapsLock" => "⇪ Caps",
        "PrintScreen" => "󰹑 ps",
        _ => raw,
    }
}

/// Resolves a printable or symbolic label for a given key based on layout.
///
/// This function is layout-aware and used when Shift is held to produce
/// the correct symbols on UK/US keyboards.
///
/// # Arguments
/// * `key` - The rdev `Key` to interpret.
/// * `layout` - The detected `KeyboardLayout`.
///
/// # Returns
/// * A `String` representing the resolved key symbol.
pub fn resolve_key_label(key: Key, layout: &KeyboardLayout) -> String {
    match layout {
        KeyboardLayout::UnitedKingdom => resolve_uk_label(key),
        KeyboardLayout::UnitedStates => resolve_us_label(key),
        _ => resolve_us_label(key), // fallback
    }
}

/// Resolves the correct shifted US keyboard symbol for a key.
fn resolve_us_label(key: Key) -> String {
    use Key::*;
    match key {
        Num1 => "!".to_string(),
        Num2 => "@".to_string(),
        Num3 => "#".to_string(),
        Num4 => "$".to_string(),
        Num5 => "%".to_string(),
        Num6 => "^".to_string(),
        Num7 => "&".to_string(),
        Num8 => "*".to_string(),
        Num9 => "(".to_string(),
        Num0 => ")".to_string(),
        Minus => "_".to_string(),
        Equal => "+".to_string(),
        BackQuote => "~".into(),
        Quote => "\"".into(),
        BackSlash | IntlBackslash => "|".into(),
        k => normalize_key_label(&format!("{:?}", k)).to_string(),
    }
}

/// Resolves the correct shifted UK keyboard symbol for a key.
fn resolve_uk_label(key: Key) -> String {
    use Key::*;
    match key {
        Num1 => "!".to_string(),
        Num2 => "\"".to_string(),
        Num3 => "£".to_string(),
        Num4 => "$".to_string(),
        Num5 => "%".to_string(),
        Num6 => "^".to_string(),
        Num7 => "&".to_string(),
        Num8 => "*".to_string(),
        Num9 => "(".to_string(),
        Num0 => ")".to_string(),
        Minus => "_".to_string(),
        Equal => "+".to_string(),
        Quote => "\"".into(),
        BackSlash | IntlBackslash => "|".into(),
        k => normalize_key_label(&format!("{:?}", k)).to_string(),
    }
}
