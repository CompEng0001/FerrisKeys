    use crate::{
    ui::visualiser::VisualiserApp,
    input::input::{start_input_listener, InputEvent},
    config::config::{Config, setup_custom_fonts},
};
use std::{
    thread,
    sync::{mpsc, Arc},
};
use eframe::egui::{self, ViewportCommand};

pub fn run() -> Result<(), eframe::Error> {
    let (tx, rx) = mpsc::channel::<InputEvent>();

    // Start background input listener
    thread::spawn(move || {
        start_input_listener(tx);
    });


    let config = Config::load_auto();

    let app = VisualiserApp::new(config.clone(), rx);


    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/images/FerrisKeys.ico"))
        .expect("The icon data must be valid");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_inner_size(config.size)
            .with_position(config.position)
            .with_icon(Arc::new(icon)),
        ..Default::default()
    };

    eframe::run_native(
        "FerrisKeys",
        options,
        Box::new(move |cc| {
            setup_custom_fonts(&cc.egui_ctx);
            cc.egui_ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(true));
            Ok(Box::new(app))
        }),
    )
}
