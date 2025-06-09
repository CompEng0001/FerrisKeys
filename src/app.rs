use crate::{
    config::config::{setup_custom_fonts, Config},
    input::input::{start_input_listener, InputEvent},
    ui::visualiser::VisualiserApp,
};
use eframe::egui::{self, ViewportCommand};
use std::{
    sync::{mpsc, Arc},
    thread,
};

/// Launches the FerrisKeys visualizer application.
///
/// - Spawns a background thread to listen for keyboard/mouse input events.
/// - Loads the user configuration, including window size, position, and fonts.
/// - Sets up a transparent, always-on-top window with no decorations.
/// - Initializes the `VisualiserApp`, passing in the input event receiver channel.
/// - Configures the GUI context, including font overrides and mouse passthrough.
///
/// # Returns
/// `Ok(())` if the app launches and exits successfully, or `Err(eframe::Error)` if startup fails.
pub fn run() -> Result<(), eframe::Error> {
    // Create a channel for transmitting input events between threads
    let (tx, rx) = mpsc::channel::<InputEvent>();

    // Spawn the input listener in a background thread
    thread::spawn(move || {
        start_input_listener(tx);
    });

    // Load configuration from disk (or fallback to defaults)
    let config = Config::load_auto();

    // Construct the visualiser app with config and input event receiver
    let app = VisualiserApp::new(config.clone(), rx);

    // Load application icon from embedded PNG byte data
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/images/FerrisKeys.ico"))
        .expect("The icon data must be valid");

    // Define window options including size, position, transparency, etc.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) // No window borders or titlebar
            .with_transparent(true) // Transparent background
            .with_always_on_top() // Keep window above others
            .with_inner_size(config.size) // Initial window size
            .with_position(config.position) // Initial window position
            .with_icon(Arc::new(icon)), // Window/taskbar icon
        ..Default::default()
    };

    // Run the application using `eframe`, setting up the GUI context and app lifecycle
    eframe::run_native(
        "FerrisKeys",
        options,
        Box::new(move |cc| {
            setup_custom_fonts(&cc.egui_ctx); // Load user/custom fonts
            cc.egui_ctx
                .send_viewport_cmd(ViewportCommand::MousePassthrough(true)); // Allow clicks to pass through
            Ok(Box::new(app))
        }),
    )
}
