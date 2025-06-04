use crate::config::config::Config;
use crate::input::keymap::{
    category_for_key,
    normalize_key_label,
    normalize_mouse_label,
    KeyCategory::*,
};

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use eframe::egui::{
    self,
    FontId,
    Pos2
};

#[derive(Clone)]
pub struct KeyEntry {
    pub icon: String,
    pub label: String,
    pub anim: f32,
    pub time: Instant,
}

pub struct KeyBuffer {
    pub keys: VecDeque<KeyEntry>,
}

impl KeyBuffer {
    pub fn new() -> Self {
        Self {
            keys: VecDeque::new(),
        }
    }

    pub fn push_key(&mut self, _unused_icon: &str, label: &str, mouse: bool) {
        if let Some(existing) = self.keys.iter_mut().find(|k| k.label == label) {
            existing.time = Instant::now();
            existing.anim = 0.8;
            return;
        }

        let raw : String;
        if !mouse {
            raw = normalize_key_label(label).to_string();
        }
        else{
            raw = normalize_mouse_label(label).to_string();
        }
        let label = match &raw {
            l if l.starts_with("Key") => &l[3..],
            l if l.starts_with("Num") && l.len() == 4 && l[3..].chars().all(|c| c.is_ascii_digit()) => &l[3..],
            _ => &raw,
        };

        let (icon, label_text) = if let Some(space_idx) = label.find(' ') {
            label.split_at(space_idx)
        } else {
            ("", label)
        };

        let icon = icon.trim();
        let label_clean = label_text.trim();

        let formatted_label = if label_clean.to_lowercase().starts_with("f") {
            label_clean.to_uppercase()
        } else {
            label_clean.to_string()
        };

        self.keys.push_back(KeyEntry {
            icon: icon.to_string(),
            label: formatted_label,
            anim: 0.8,
            time: Instant::now(),
        });
    }

    pub fn render(&mut self, ui: &mut egui::Ui, config: &Config, max_width: f32) {
        let padding = 8.0;
        let mut total_width = 0.0;
        let mut draw_list = vec![];

        let now = Instant::now();
        self.keys.retain(|k| now.duration_since(k.time) < Duration::from_secs(1));

        for key in self.keys.iter_mut().rev() {
            let category = category_for_key(&key.label);
            let style = config.styles
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

        draw_list.reverse();
        let mut x = ui.max_rect().right() - total_width;

        for key in &draw_list {
            let category = category_for_key(&key.label);
            let style = config.styles
                .get(&category)
                .cloned()
                .unwrap_or_else(Config::fallback_style);

            let scale = key.anim.min(1.0);
            let size = egui::vec2(style.width * scale, style.height * scale);
            let top_left = egui::pos2(
                x + (style.width - size.x) / 2.0,
                (style.height - size.y) / 2.0,
            );
            let rect = egui::Rect::from_min_size(top_left, size);
            let painter = ui.painter_at(rect);

            painter.rect_filled(rect, egui::CornerRadius::same(8), style.bg_color);

            let icon_text = &key.icon;
            let main_text = &key.label;

            match category {
                Normal | Numeric | Symbol | Navigation => {
                    painter.text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        main_text,
                        FontId::proportional(style.text_size),
                        style.fg_color,
                    );
                }
                Function => {
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

            x += style.width + padding;
        }

        let allowed_count = draw_list.len();
        while self.keys.len() > allowed_count {
            self.keys.pop_front();
        }
    }
}
