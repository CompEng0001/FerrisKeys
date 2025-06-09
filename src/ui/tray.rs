use std::{path::PathBuf, process::Command, thread};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIcon, TrayIconBuilder,
};

#[cfg(target_os = "windows")]
use crate::platform::windows::tray::load_embedded_icon;

#[cfg(not(target_os = "windows"))]
/// Returns a fallback 1x1 transparent icon for non-Windows platforms,
/// as the tray icon feature is currently only active on Windows.
fn load_embedded_icon() -> tray_icon::Icon {
    tray_icon::Icon::from_rgba(vec![0; 4], 1, 1).unwrap()
}

/// Determines the configuration directory path for the current platform.
///
/// - On Windows: uses `C:\Users\<User>\AppData\Roaming\ferriskeys`
/// - On Unix-like systems: uses `$XDG_CONFIG_HOME/ferriskeys`
///
/// # Returns
/// An `Option<PathBuf>` pointing to the `ferriskeys` configuration directory.
fn get_config_path() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        dirs::data_dir().map(|d| d.join("ferriskeys"))
    } else {
        dirs::config_dir().map(|d| d.join("ferriskeys"))
    }
}

/// Opens the user's configuration folder in the default file explorer.
///
/// Attempts to use platform-specific commands:
/// - Windows: `explorer`
/// - macOS: `open`
/// - Linux/others: `xdg-open`
fn open_config_folder() {
    let Some(config_dir) = get_config_path() else {
        eprintln!("Could not determine config path");
        return;
    };

    let _ = if cfg!(target_os = "windows") {
        Command::new("explorer").arg(config_dir).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(config_dir).spawn()
    } else {
        Command::new("xdg-open").arg(config_dir).spawn()
    };
}

#[cfg(target_os = "windows")]
/// Spawns a system tray icon with a menu for `FerrisKeys` on Windows.
///
/// The tray menu includes:
/// - **"Open Config"**: Opens the configuration directory in a file explorer.
/// - **"Quit"**: Terminates the application.
///
/// The function launches a background thread to listen for menu item events.
///
/// # Returns
/// `Some(TrayIcon)` if the tray icon was successfully created; `None` otherwise.
pub fn spawn_tray() -> Option<TrayIcon> {
    if get_config_path().is_none() {
        eprintln!("Could not determine config path");
        return None;
    }

    // Create tray menu items
    let open_item = MenuItem::new("Open Config", true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    let open_id = open_item.id().clone();
    let quit_id = quit_item.id().clone();

    // Build the menu and append items
    let menu = Menu::new();
    menu.append(&open_item).unwrap();
    menu.append(&quit_item).unwrap();

    // Build the tray icon with the specified menu and tooltip
    let tray_icon = TrayIconBuilder::new()
        .with_icon(load_embedded_icon())
        .with_menu(Box::new(menu))
        .with_tooltip("FerrisKeys")
        .build()
        .expect("Could not create tray icon");

    // Listen for menu events in a background thread
    let rx = MenuEvent::receiver();
    thread::spawn(move || {
        for event in rx.iter() {
            if event.id == open_id {
                open_config_folder();
            } else if event.id == quit_id {
                std::process::exit(0);
            }
        }
    });

    Some(tray_icon)
}
