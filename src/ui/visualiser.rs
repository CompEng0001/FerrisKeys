use crate::input::{
    input::InputEvent,
    keymap::{normalize_key_label, normalize_mouse_label},
    layout::{
        detect_layout, 
        KeyboardLayout
    },
};
use crate::config::config::Config;
use crate::ui::ui::KeyBuffer;

use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use eframe::{
    egui,
    egui::{
        Context, 
        Color32, 
        CentralPanel, 
        Frame, 
        Visuals, 
        Rgba,
        ViewportCommand,
    }, 
    App
};

pub struct VisualiserApp {
    pub config: Config,
    pub rx: Receiver<InputEvent>,
    pub key_buffer: KeyBuffer,
    pub recently_seen: HashSet<String>,
    pub last_clear: Instant,
    pub last_ui_width: f32,
    pub layout:  KeyboardLayout,
}

impl VisualiserApp {
    pub fn new( config: Config, rx: Receiver<InputEvent>) -> Self {
        Self {
            config,
            rx,
            key_buffer: KeyBuffer::new(),
            recently_seen: HashSet::new(),
            last_clear: Instant::now(),
            last_ui_width: 0.0,
            layout: detect_layout(),
        }
    }
}

impl App for VisualiserApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if self.config.maybe_reload() {
            ctx.send_viewport_cmd(ViewportCommand::OuterPosition(egui::pos2(
                self.config.position[0],
                self.config.position[1],
            )));
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(egui::vec2(
                self.config.size[0],
                self.config.size[1],
            )));
            ctx.send_viewport_cmd(ViewportCommand::Focus);
            ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(true))
        }

        let mut needs_repaint = false;

        while let Ok(event) = self.rx.try_recv() {
            match event {
                InputEvent::KeyPress(label) => {
                    let normalised = normalize_key_label(&label).to_string();
                    if !self.recently_seen.contains(&normalised) {
                        self.key_buffer.push_key("", &normalised, false);
                        self.recently_seen.insert(normalised);
                        needs_repaint = true;
                    }
                }
                 InputEvent::MouseClick(label) => {
                    let normalised = normalize_mouse_label(&label);
                    if !self.recently_seen.contains(normalised) {
                        self.key_buffer.push_key("", &normalised, true);
                        self.recently_seen.insert(label);
                        needs_repaint = true;
                    }
                }
            }
        }

        if self.last_clear.elapsed() > Duration::from_millis(250) {
            self.recently_seen.clear();
            self.last_clear = Instant::now();
        }

        CentralPanel::default()
            .frame(Frame::NONE.fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let width = ui.available_width();
                self.last_ui_width = width;
                self.key_buffer.render(ui, &self.config, width);
            });

        if needs_repaint {
            ctx.request_repaint();
        } else {
            ctx.request_repaint_after(Duration::from_millis(33));
        }
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }
}
