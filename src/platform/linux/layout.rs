use crate::input::layout::KeyboardLayout;
use std::process::Command;

/// Detects the active keyboard layout on Linux using the `setxkbmap -query` command.
///
/// The function attempts to run `setxkbmap -query`, which outputs lines like:
/// ```
/// layout:     us,gb
/// variant:    ,
/// ```
///
/// It parses the `layout:` line and uses the **first layout** in the list to determine the active one.
///
/// Currently supports:
/// - `"gb"` → `KeyboardLayout::UnitedKingdom`
/// - `"us"` → `KeyboardLayout::UnitedStates`
///
/// Unknown layouts are returned as `KeyboardLayout::Other(0)`. The `0` is a placeholder and may be
/// enhanced later to carry actual layout IDs or hashes.
///
/// # Returns
/// A `KeyboardLayout` enum corresponding to the active layout, or a fallback if detection fails.
pub fn detect_layout() -> KeyboardLayout {
    if let Ok(output) = Command::new("setxkbmap").arg("-query").output() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            for line in stdout.lines() {
                if line.starts_with("layout:") {
                    let layout = line.split(':').nth(1).unwrap_or("").trim();

                    // If multiple layouts are listed (e.g., "gb,us"), use the first one
                    let primary_layout = layout.split(',').next().unwrap_or("").trim();

                    return match primary_layout {
                        "gb" => KeyboardLayout::UnitedKingdom,
                        "us" => KeyboardLayout::UnitedStates,
                        _ => KeyboardLayout::Other(0), // Unrecognized layout
                    };
                }
            }
        }
    }

    // Fallback if command fails or output is malformed
    KeyboardLayout::Other(0)
}
