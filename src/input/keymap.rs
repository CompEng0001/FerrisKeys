use rdev::Key;

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

pub fn category_for_key(key: &str) -> KeyCategory {
    match key.to_ascii_lowercase().as_str() {
        "󰍽" |"left" | "right" | "middle" => KeyCategory::Mouse,
         "meta" | "esc" | "escape" | "\u{f0206} esc" => KeyCategory::Escape,
        "ctrl" | "control" | "⌃ control" |
        "shift" | "⇧ shift" |
        "alt" | "⌥ alt" |
        "tab"| "num" | "numlock" | "caps" => KeyCategory::Modifier,

        | "󰹑" | "ps" | "backspace" | "delete" | "del" | "back" | "ins" | "insert" => KeyCategory::Editor,

         "↑" | "↓" | "←" | "→" => KeyCategory::Navigation,

        "home" | "end" | "pageup" | "pagedown" | "pgup" | "pgdn" | "scroll" | "scrollock" => KeyCategory::Scrollable,

        "space" | "󱁐 space" => KeyCategory::Space,

        "<" | ">" | "|" |"£"|"$"|"%"|"^" |"&" |"_" |"¬" | "#" | "`" | "(" | ")" | "@" | "+" | "-" | "=" | "*" |  "\\" | "/" | "," | "." | ";" | ":" | "!" | "'" | "[" | "]" | "?" | "~" | "\"" => KeyCategory::Symbol,
        
        k if k.starts_with('f') && k[1..].parse::<u8>().map_or(false, |n| (1..=24).contains(&n)) => KeyCategory::Function,

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
          || k.contains("home") => KeyCategory::AltFunction,

        k if k.chars().all(|c| c.is_ascii_digit()) => KeyCategory::Numeric,

        k if k.chars().all(|c| c.is_ascii_alphabetic()) => KeyCategory::Normal,

        _ => KeyCategory::Unknown,
    }
}

pub fn normalize_mouse_label(raw: &str) -> &str {
    //println!("nml:ButtonB4M: {:?}", raw);
    match raw {
        "MouseLeft" => "󰍽 left" ,
        "MouseRight" => "󰍽 right ",
        "MouseMiddle" => "󰍽 middle",
        _ => raw
    }
    
}

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
        "BackSlash" => "\\",
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

pub fn resolve_key_label(key: Key) -> String {
    println!("key: {:?}",key);
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
        LeftBracket => "{".to_string(),
        RightBracket => "}".to_string(),
        BackQuote => "@".to_string(),
        BackSlash => "|".to_string(),
        SemiColon => ":".to_string(),
        Quote => "~".to_string(),
        Comma => "<".to_string(),
        Dot => ">".to_string(),
        Slash => "?".to_string(),
        Unknown(223) => "¬".to_string(),
        Unknown(12) => "".to_string(),

        // fallback to unshifted
        k => normalize_key_label(&format!("{:?}", k)).to_string(),
    }
}

