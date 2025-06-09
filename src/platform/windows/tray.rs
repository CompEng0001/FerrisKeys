use tray_icon::Icon;

/// Loads the embedded tray icon from the application resources (Windows only).
///
/// This function retrieves an icon resource embedded in the `.exe` file using its resource ID.
/// The resource must be defined in the application’s `app.rc` file or build configuration
/// with ID `1`.
///
/// # Returns
/// * `Icon` — A `tray_icon::Icon` object that can be passed to `TrayIconBuilder`.
///
/// # Panics
/// * If the icon resource is not found or cannot be loaded, the function panics with
///   `"Failed to load embedded icon"`.
///
/// # Platform
/// This function is only compiled on Windows (`#[cfg(target_os = "windows")]`).
#[cfg(target_os = "windows")]
pub fn load_embedded_icon() -> tray_icon::Icon {
    Icon::from_resource(1, None).expect("Failed to load embedded icon")
}
