// Prevents a console window from opening on Windows GUI apps
#![windows_subsystem = "windows"]

// Module declarations
mod app;
mod config;
mod input;
mod platform;
mod ui;

/// Detects problematic Wayland environments that are known to cause issues with window creation.
/// Specifically checks for Raspberry Pi setups where Glutin fails under Wayland.
///
/// # Returns
/// `true` if Wayland is in use and it's likely to cause issues.
/// `false` otherwise.
fn is_problematic_wayland() -> bool {
    std::env::var("XDG_SESSION_TYPE").map_or(false, |v| v == "wayland")
        && std::env::var("WAYLAND_DISPLAY").is_ok()
}

/// Entry point of the application.
///
/// - Ensures the user configuration file exists, creating one from defaults if missing.
/// - On Windows, initializes a system tray icon.
/// - Exits early with a message if a known problematic Wayland setup is detected.
/// - Runs the main application loop via `app::run()`.
/// - On failure, prints an error and exits with a non-zero status.
fn main() {
    // Ensure configuration file is present or create it from defaults
    config::config::Config::ensure_config_exists().expect("Failed to write config");

    // Spawn the system tray icon on Windows
    #[cfg(target_os = "windows")]
    let tray_icon = ui::tray::spawn_tray();

    // Check for problematic Wayland setup (e.g., on Raspberry Pi)
    if is_problematic_wayland() {
        eprintln!("Wayland detected and native window creation may be unsupported on this system.");
        eprintln!("Try launching with:");
        eprintln!("    LIBGL_ALWAYS_SOFTWARE=1 ./ferriskeys");
        eprintln!("Or use an X11 session instead.");
        std::process::exit(1);
    }

    // Attempt to run the application
    if let Err(err) = app::run() {
        eprintln!("Error: {:#?}", err);

        // Clean up tray icon if on Windows
        #[cfg(target_os = "windows")]
        drop(tray_icon);

        std::process::exit(1);
    }
}
