use crate::config::config::Config;
use crate::input::input::InputEvent;
use crate::ui::ui::KeyBuffer;

use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use eframe::{
    egui,
    egui::{CentralPanel, Color32, Context, Frame, Rgba, ViewportCommand, Visuals},
    App,
};

/// The main egui application struct responsible for rendering and updating
/// the live input visualisation overlay.
pub struct VisualiserApp {
    pub config: Config,           // User configuration (position, size, styles, etc.)
    pub rx: Receiver<InputEvent>, // Channel receiver for input events (keys, mouse)
    pub key_buffer: KeyBuffer,    // Circular buffer of visible keys to render
    pub recently_seen: HashSet<String>, // Used to debounce repeat events within short intervals
    pub last_clear: Instant,      // Timer for clearing the recently_seen cache
    pub last_ui_width: f32,       // Tracks the last available UI width (used for layout)
}

impl VisualiserApp {
    /// Creates a new instance of the visualiser app with the given config and input receiver.
    pub fn new(config: Config, rx: Receiver<InputEvent>) -> Self {
        Self {
            config,
            rx,
            key_buffer: KeyBuffer::new(),
            recently_seen: HashSet::new(),
            last_clear: Instant::now(),
            last_ui_width: 0.0,
        }
    }
}

impl App for VisualiserApp {
    /// Called every frame to update the application state and render the UI.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Reload config if the file has changed on disk
        if self.config.maybe_reload() {
            // Reapply size, position, focus, and mouse passthrough
            ctx.send_viewport_cmd(ViewportCommand::OuterPosition(egui::pos2(
                self.config.position[0],
                self.config.position[1],
            )));
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(egui::vec2(
                self.config.size[0],
                self.config.size[1],
            )));
            ctx.send_viewport_cmd(ViewportCommand::Focus);
            ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(true));
        }

        let mut needs_repaint = false;

        // Handle all available input events from the background listener
        while let Ok(event) = self.rx.try_recv() {
            match event {
                InputEvent::KeyPress(label) => {
                    if !self.recently_seen.contains(&label) {
                        self.key_buffer.push_key("", &label, false);
                        self.recently_seen.insert(label);
                        needs_repaint = true;
                    }
                }
                InputEvent::MouseClick(label) => {
                    if !self.recently_seen.contains(&label) {
                        self.key_buffer.push_key("", &label, true);
                        self.recently_seen.insert(label);
                        needs_repaint = true;
                    }
                }
            }
        }

        // Debounce key repeat events every 250ms
        if self.last_clear.elapsed() > Duration::from_millis(250) {
            self.recently_seen.clear();
            self.last_clear = Instant::now();
        }

        // Draw the transparent central panel with all active keys
        CentralPanel::default()
            .frame(Frame::NONE.fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let width = ui.available_width();
                self.last_ui_width = width;
                self.key_buffer.render(ui, &self.config, width);
            });

        // Request immediate repaint if we received an event; otherwise throttle
        if needs_repaint {
            ctx.request_repaint();
        } else {
            ctx.request_repaint_after(Duration::from_millis(33)); // ~30fps idle refresh
        }
    }

    /// Returns the clear color of the background — fully transparent.
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }
}
