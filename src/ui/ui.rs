use crate::config::config::Config;
use crate::input::keymap::{
    category_for_key, normalize_key_label, normalize_mouse_label, KeyCategory::*,
};

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use eframe::egui::{self, FontId, Pos2};

/// Represents a single key or mouse input event for visualization.
#[derive(Clone)]
pub struct KeyEntry {
    pub icon: String,  // Optional icon string (e.g., modifier or mouse icon)
    pub label: String, // Main label text (e.g., "Ctrl", "A", "F5")
    pub anim: f32,     // Animation progress (0.0 to 1.0)
    pub time: Instant, // Time of last event (for fading/removal)
}

/// A ring buffer of currently visible keys to render onscreen.
///
/// Used to store and display a limited set of recent inputs.
pub struct KeyBuffer {
    pub keys: VecDeque<KeyEntry>,
}

impl KeyBuffer {
    /// Creates a new, empty key buffer.
    pub fn new() -> Self {
        Self {
            keys: VecDeque::new(),
        }
    }

    /// Adds a new key or mouse input to the buffer.
    ///
    /// - Prevents duplicate key labels by refreshing existing ones.
    /// - Normalizes labels using platform-specific logic.
    /// - Parses label into icon + text if applicable (e.g., `"⇧ Shift"`).
    pub fn push_key(&mut self, _unused_icon: &str, label: &str, mouse: bool) {
        // Check if label already exists and refresh its time/animation if found
        if let Some(existing) = self.keys.iter_mut().find(|k| k.label == label) {
            existing.time = Instant::now();
            existing.anim = 0.8;
            return;
        }

        // Normalize the input label for consistency
        let raw = if !mouse {
            normalize_key_label(label).to_string()
        } else {
            normalize_mouse_label(label).to_string()
        };

        // Strip known key prefixes for better UI clarity
        let label = match &raw {
            l if l.starts_with("Key") => &l[3..],
            l if l.starts_with("Num")
                && l.len() == 4
                && l[3..].chars().all(|c| c.is_ascii_digit()) =>
            {
                &l[3..]
            }
            _ => &raw,
        };

        // Attempt to split icon and label by the first space
        let (icon, label_text) = if let Some(space_idx) = label.find(' ') {
            label.split_at(space_idx)
        } else {
            ("", label)
        };

        let icon = icon.trim();
        let label_clean = label_text.trim();

        // Format label text: e.g., F1, F12 stay uppercase, others retain formatting
        let formatted_label = if label_clean.to_lowercase().starts_with("f") {
            label_clean.to_uppercase()
        } else {
            label_clean.to_string()
        };

        // Add the newly created entry to the buffer
        self.keys.push_back(KeyEntry {
            icon: icon.to_string(),
            label: formatted_label,
            anim: 0.8,
            time: Instant::now(),
        });
    }

    /// Renders the current key buffer onto the provided `egui` UI panel.
    ///
    /// - Applies per-key styles and animation.
    /// - Clips the display based on available width.
    /// - Automatically expires keys older than 1 second.
    pub fn render(&mut self, ui: &mut egui::Ui, config: &Config, max_width: f32) {
        let padding = 8.0;
        let mut total_width = 0.0;
        let mut draw_list = vec![];

        // Remove expired keys (older than 1 second)
        let now = Instant::now();
        self.keys
            .retain(|k| now.duration_since(k.time) < Duration::from_secs(1));

        // Determine which keys can fit on the screen from right to left
        for key in self.keys.iter_mut().rev() {
            let category = category_for_key(&key.label);
            let style = config
                .styles
                .get(&category)
                .cloned()
                .unwrap_or_else(Config::fallback_style);

            let width = style.width + padding;

            if total_width + width > max_width {
                break;
            }

            if key.anim < 1.0 {
                key.anim += 0.1;
            }

            total_width += width;
            draw_list.push(key.clone());
        }

        // Draw from left to right (restore original order)
        draw_list.reverse();
        let mut x = ui.max_rect().right() - total_width;

        // Render each key visual
        for key in &draw_list {
            let category = category_for_key(&key.label);
            let style = config
                .styles
                .get(&category)
                .cloned()
                .unwrap_or_else(Config::fallback_style);

            // Apply animation scaling
            let scale = key.anim.min(1.0);
            let size = egui::vec2(style.width * scale, style.height * scale);
            let top_left = egui::pos2(
                x + (style.width - size.x) / 2.0,
                (style.height - size.y) / 2.0,
            );
            let rect = egui::Rect::from_min_size(top_left, size);
            let painter = ui.painter_at(rect);

            // Background
            painter.rect_filled(rect, egui::CornerRadius::same(8), style.bg_color);

            let icon_text = &key.icon;
            let main_text = &key.label;

            // Render logic by category
            match category {
                Normal | Numeric | Symbol | Navigation | Function => {
                    painter.text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        main_text,
                        FontId::proportional(style.text_size),
                        style.fg_color,
                    );
                }
                Modifier => {
                    if !icon_text.is_empty() {
                        painter.text(
                            Pos2::new(rect.right() - 10.0, rect.top() + 10.0),
                            egui::Align2::RIGHT_TOP,
                            icon_text,
                            FontId::proportional(style.icon_size),
                            style.fg_color,
                        );
                    }

                    painter.text(
                        Pos2::new(rect.right() - 10.0, rect.bottom() - 10.0),
                        egui::Align2::RIGHT_BOTTOM,
                        main_text,
                        FontId::proportional(style.text_size),
                        style.fg_color,
                    );
                }
                Scrollable | Editor | Escape | AltFunction | Mouse => {
                    if !icon_text.is_empty() {
                        painter.text(
                            Pos2::new(rect.right() - 47.5, rect.top() + 20.0),
                            egui::Align2::CENTER_CENTER,
                            icon_text,
                            FontId::proportional(style.icon_size),
                            style.fg_color,
                        );
                    }

                    painter.text(
                        Pos2::new(rect.right() - 45.0, rect.bottom() - 20.0),
                        egui::Align2::CENTER_CENTER,
                        main_text,
                        FontId::proportional(style.text_size),
                        style.fg_color,
                    );
                }
                _ => {
                    if !icon_text.is_empty() {
                        painter.text(
                            Pos2::new(rect.center().x, rect.top() + 18.0),
                            egui::Align2::CENTER_CENTER,
                            icon_text,
                            FontId::proportional(style.icon_size),
                            style.fg_color,
                        );
                    }

                    painter.text(
                        Pos2::new(rect.center().x, rect.bottom() - 26.0),
                        egui::Align2::CENTER_CENTER,
                        main_text,
                        FontId::proportional(style.text_size),
                        style.fg_color,
                    );
                }
            }

            // Advance drawing position for next key
            x += style.width + padding;
        }

        // Trim excess keys from buffer that didn't fit onscreen
        let allowed_count = draw_list.len();
        while self.keys.len() > allowed_count {
            self.keys.pop_front();
        }
    }
}
