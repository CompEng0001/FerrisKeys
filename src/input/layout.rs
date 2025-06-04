#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardLayout {
    UnitedStates,
    UnitedKingdom,
    German,
    French,
    Italian,
    Japanese,
    Korean,
    Other(u16),
}

pub fn detect_layout() -> KeyboardLayout {
    #[cfg(target_os = "windows")]
    use winapi::shared::minwindef::HKL;
    use winapi::um::processthreadsapi::GetCurrentThreadId;
    use winapi::um::winuser::GetKeyboardLayout;
    unsafe {
        let thread_id = GetCurrentThreadId();
        let hkl: HKL = GetKeyboardLayout(thread_id);
        let layout_id = ((hkl as usize >> 16) & 0xFFFF) as u16;
        //println!("{:?}",GetKeyboardLayout(0));
        match layout_id {
            0x0809 => KeyboardLayout::UnitedKingdom,
            0x0409 => KeyboardLayout::UnitedStates,
            0x0407 => KeyboardLayout::German,
            0x040C => KeyboardLayout::French,
            0x0410 => KeyboardLayout::Italian,
            0x0411 => KeyboardLayout::Japanese,
            0x0412 => KeyboardLayout::Korean,
            _ => KeyboardLayout::Other(layout_id),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        KeyboardLayout::UnitedStates // Default fallback
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = Command::new("setxkbmap").arg("-query").output() {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                for line in stdout.lines() {
                    if line.starts_with("layout:") {
                        let layout = line.split(':').nth(1).unwrap_or("").trim();
                        return match layout {
                            "gb" => KeyboardLayout::Uk,
                            "us" => KeyboardLayout::Us,
                            other => KeyboardLayout::Other(other.to_string()),
                        };
                    }
                }
            }
        }
        KeyboardLayout::Other("unknown".into())
    }

    #[cfg(target_os = "macos")]
    {
        use std::fs;
        use plist::Value;
        let plist_path = "~/Library/Preferences/com.apple.HIToolbox.plist";
        let expanded = shellexpand::tilde(plist_path);
        if let Ok(data) = fs::read(expanded.to_string()) {
            if let Ok(plist) = Value::from_reader_xml(&*data) {
                if let Some(array) = plist.as_dictionary()
                    .and_then(|d| d.get("AppleSelectedInputSources"))
                    .and_then(|v| v.as_array())
                {
                    for item in array {
                        if let Some(layout) = item.as_dictionary()
                            .and_then(|m| m.get("InputSourceID"))
                            .and_then(|v| v.as_string())
                        {
                            if layout.contains("British") {
                                return KeyboardLayout::Uk;
                            }
                            if layout.contains("US") {
                                return KeyboardLayout::Us;
                            }
                        }
                    }
                }
            }
        }
        KeyboardLayout::Other("unknown".into())
    }
}
