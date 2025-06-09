use crate::config::default_config;
use crate::input::keymap::KeyCategory;

use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily};
use notify::{Config as NotifyConfig, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, SystemTime};
use toml::Value;
/// A visual style definition for a specific key category.
#[derive(Debug, Clone)]
pub struct Style {
    /// Width of the key display box.
    pub width: f32,
    /// Height of the key display box.
    pub height: f32,
    /// Font size used for the icon (e.g., modifier symbol).
    pub icon_size: f32,
    /// Font size used for the main key label.
    pub text_size: f32,
    /// Background color of the key box.
    pub bg_color: Color32,
    /// Foreground (text/icon) color.
    pub fg_color: Color32,
}

/// Runtime configuration for FerrisKeys, loaded from `config.toml`.
#[derive(Debug)]
pub struct Config {
    /// Map of styles by key category.
    pub styles: HashMap<KeyCategory, Style>,
    /// Timeout for showing key presses (in milliseconds).
    pub timeout_ms: u64,
    /// Position of the overlay window (x, y).
    pub position: [f32; 2],
    /// Size of the overlay window (width, height).
    pub size: [f32; 2],
    /// Path to the loaded configuration file.
    pub path: String,
    /// Timestamp of last modification to the config file.
    pub last_modified: Option<SystemTime>,
    /// Optional file watcher event channel for hot-reloading.
    #[allow(clippy::type_complexity)]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub reload_rx: Option<Receiver<()>>,
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            styles: self.styles.clone(),
            timeout_ms: self.timeout_ms,
            position: self.position,
            size: self.size,
            path: self.path.clone(),
            last_modified: self.last_modified,
            reload_rx: None, // cloned configs do not inherit watchers
        }
    }
}

impl Config {
    /// Ensures that a valid configuration file exists, creating one from defaults if missing.
    pub fn ensure_config_exists() -> std::io::Result<()> {
        let paths = Config::config_paths();

        println!("🔍 Checking config paths:");
        for p in &paths {
            println!("  - {}", p.display());
        }

        let path = paths.iter().find(|p| p.to_str().is_some()).unwrap();

        if !path.exists() {
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)?;
            }

            std::fs::write(path, default_config::DEFAULT_CONFIG_TOML)?;
            println!("Created config at: {}", path.display());
        } else {
            println!("Config already exists at: {}", path.display());
        }

        Ok(())
    }

    /// Loads the first available config file from preferred paths.
    /// If none exist, returns default settings.
    pub fn load_auto() -> Self {
        let paths = Config::config_paths();

        for path in &paths {
            if path.exists() {
                return Config::load(path.to_str().unwrap());
            }
        }

        Config::default()
    }

    /// Returns a list of locations to look for `config.toml`.
    /// - System-specific config dir (e.g., `$HOME/.config/ferriskeys`)
    /// - Fallback to current directory
    fn config_paths() -> Vec<std::path::PathBuf> {
        let mut paths = vec![];

        if cfg!(target_os = "windows") {
            if let Some(appdata) = std::env::var_os("APPDATA") {
                paths.push(Path::new(&appdata).join("ferriskeys").join("config.toml"));
            }
        } else if let Some(home) = std::env::var_os("HOME") {
            paths.push(Path::new(&home).join(".config/ferriskeys/config.toml"));
        }

        if let Ok(cwd) = std::env::current_dir() {
            paths.push(cwd.join("config.toml"));
        }

        paths
    }

    /// Loads a configuration file from the given path and parses styles, size, position, etc.
    pub fn load(path: &str) -> Self {
        let mut styles = Self::fallback_styles();
        let mut timeout_ms = 1200;
        let mut position = [500.0, 500.0];
        let mut size = [800.0, 120.0];
        let path_obj = Path::new(path);
        let last_modified = fs::metadata(path_obj).and_then(|m| m.modified()).ok();

        if let Ok(content) = fs::read_to_string(path_obj) {
            if let Ok(toml) = content.parse::<Value>() {
                if let Some(win) = toml.get("window") {
                    if let Some(arr) = win.get("position").and_then(|v| v.as_array()) {
                        if arr.len() == 2 {
                            position = [
                                arr[0].as_float().unwrap_or(100.0) as f32,
                                arr[1].as_float().unwrap_or(100.0) as f32,
                            ];
                        }
                    }
                    if let Some(arr) = win.get("size").and_then(|v| v.as_array()) {
                        if arr.len() == 2 {
                            size = [
                                arr[0].as_float().unwrap_or(800.0) as f32,
                                arr[1].as_float().unwrap_or(120.0) as f32,
                            ];
                        }
                    }
                }

                if let Some(s) = toml.get("styles") {
                    for (cat, table) in s.as_table().unwrap_or(&toml::map::Map::new()) {
                        if let Some(key_cat) = parse_category(cat) {
                            let style = parse_style(table, &key_cat);
                            styles.insert(key_cat, style);
                        }
                    }
                }

                if let Some(timeout) = toml.get("timeout_ms").and_then(|v| v.as_integer()) {
                    timeout_ms = timeout as u64;
                }
            }
        }

        let mut config = Config {
            styles,
            timeout_ms,
            position,
            size,
            path: path.to_string(),
            last_modified,
            reload_rx: None,
        };

        config.setup_watcher();
        config
    }

    /// Loads a default config with no file watching.
    pub fn default() -> Self {
        let mut c = Config::load("does-not-exist.toml");
        c.reload_rx = None;
        c
    }

    /// Sets up a filesystem watcher on the config file.
    /// Emits a signal over a channel when the file is modified.
    fn setup_watcher(&mut self) {
        let (tx, rx) = channel();
        let path = self.path.clone();

        if !Path::new(&path).exists() {
            self.reload_rx = None;
            return;
        }

        thread::spawn(move || {
            let mut watcher = RecommendedWatcher::new(
                move |res: notify::Result<notify::Event>| {
                    if let Ok(event) = res {
                        if matches!(event.kind, EventKind::Modify(_)) {
                            let _ = tx.send(());
                        }
                    }
                },
                NotifyConfig::default(),
            )
            .expect("Failed to create watcher");

            if let Err(e) = watcher.watch(Path::new(&path), RecursiveMode::NonRecursive) {
                eprintln!("⚠️ Failed to watch config file: {e}");
                return;
            }

            loop {
                thread::sleep(Duration::from_secs(3600));
            }
        });

        self.reload_rx = Some(rx);
    }

    /// Checks whether the config file has changed.
    /// If so, reloads it and updates the current instance.
    ///
    /// # Returns
    /// `true` if the config was reloaded, `false` otherwise.
    pub fn maybe_reload(&mut self) -> bool {
        let mut triggered = false;

        if let Some(rx) = &self.reload_rx {
            if rx.try_recv().is_ok() {
                triggered = true;
            }
        }

        if !triggered {
            if let Ok(modified) = fs::metadata(&self.path).and_then(|m| m.modified()) {
                if Some(modified) > self.last_modified {
                    triggered = true;
                }
            }
        }

        if triggered {
            *self = Config::load(&self.path);
            return true;
        }

        false
    }

    /// Provides a fallback visual style if a key category is missing in config.
    pub fn fallback_style() -> Style {
        Style {
            width: 90.0,
            height: 90.0,
            icon_size: 0.0,
            text_size: 24.0,
            bg_color: hex("#3c3c3c"),
            fg_color: hex("ffffff"),
        }
    }

    /// Returns the default style map for all known `KeyCategory` values.
    pub fn fallback_styles() -> HashMap<KeyCategory, Style> {
        use KeyCategory::*;
        let mut map = HashMap::new();

        let mut insert = |cat, w, h, icon, text, bg, fg| {
            map.insert(
                cat,
                Style {
                    width: w,
                    height: h,
                    icon_size: icon,
                    text_size: text,
                    bg_color: hex(bg),
                    fg_color: hex(fg),
                },
            );
        };

        insert(Normal, 90.0, 90.0, 0.0, 20.0, "#1e1e30", "#ffffff");
        insert(Modifier, 120.0, 90.0, 25.0, 18.0, "#32283c", "#ffffff");
        insert(Editor, 90.0, 90.0, 18.0, 22.0, "#3f2e2e", "#ffffff");
        insert(Navigation, 90.0, 90.0, 20.0, 22.0, "#2e3f2e", "#ffffff");
        insert(Scrollable, 90.0, 90.0, 20.0, 22.0, "#2e3f2e", "#ffffff");
        insert(Numeric, 90.0, 90.0, 0.0, 24.0, "#2e2e2e", "#ffffff");
        insert(Symbol, 90.0, 90.0, 20.0, 24.0, "#3c2e2e", "#ffffff");
        insert(Space, 260.0, 90.0, 20.0, 28.0, "#888888", "#ffffff");
        insert(Escape, 90.0, 90.0, 20.0, 22.0, "#AA1111", "#ffffff");
        insert(Unknown, 90.0, 90.0, 14.0, 22.0, "#555555", "#ffffff");
        insert(Function, 90.0, 90.0, 14.0, 22.0, "#001155", "#ffffff");
        insert(AltFunction, 90.0, 90.0, 14.0, 22.0, "#004488", "#ffffff");
        insert(Mouse, 90.0, 90.0, 0.0, 24.0, "#801155", "#ffffff");
        map
    }
}

/// Converts a `"#RRGGBB"` color string to a `Color32` value.
/// Falls back to white if the string is malformed.
fn hex(c: &str) -> Color32 {
    let cleaned = c.trim_start_matches('#');
    if cleaned.len() != 6 {
        eprintln!("Invalid color string: '{}'. Using fallback.", c);
        return Color32::WHITE;
    }

    let r = u8::from_str_radix(&cleaned[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&cleaned[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&cleaned[4..6], 16).unwrap_or(255);

    Color32::from_rgb(r, g, b)
}

/// Parses a `Style` table from TOML with fallbacks for each field.
fn parse_style(table: &Value, category: &KeyCategory) -> Style {
    let fallback = Config::fallback_styles()
        .get(category)
        .cloned()
        .unwrap_or_else(Config::fallback_style);

    let get = |k: &str| {
        table.get(k).and_then(|v| v.as_float()).unwrap_or_else(|| {
            eprintln!(
                "Missing or invalid `{}` for {:?}. Using fallback.",
                k, category
            );
            match k {
                "width" => fallback.width as f64,
                "height" => fallback.height as f64,
                "icon_size" => fallback.icon_size as f64,
                "text_size" => fallback.text_size as f64,
                _ => 0.0,
            }
        }) as f32
    };

    let get_color = |k: &str| {
        let val = table.get(k).and_then(|v| v.as_str());
        match val {
            Some(color) => {
                let cleaned = color.trim_start_matches('#');
                if cleaned.len() == 6 {
                    hex(color)
                } else {
                    eprintln!("Invalid color '{}'. Falling back.", color);
                    match k {
                        "bg_color" => fallback.bg_color,
                        "text_color" => fallback.fg_color,
                        _ => Color32::WHITE,
                    }
                }
            }
            None => {
                eprintln!(
                    "Missing color key `{}` for {:?}. Using fallback.",
                    k, category
                );
                match k {
                    "bg_color" => fallback.bg_color,
                    "text_color" => fallback.fg_color,
                    _ => Color32::WHITE,
                }
            }
        }
    };

    Style {
        width: get("width"),
        height: get("height"),
        icon_size: get("icon_size"),
        text_size: get("text_size"),
        bg_color: get_color("bg_color"),
        fg_color: get_color("fg_color"),
    }
}

/// Parses a string into a known `KeyCategory`.
fn parse_category(name: &str) -> Option<KeyCategory> {
    use KeyCategory::*;
    Some(match name.to_ascii_lowercase().as_str() {
        "escape" => Escape,
        "normal" => Normal,
        "numeric" => Numeric,
        "modifier" => Modifier,
        "editor" => Editor,
        "navigation" => Navigation,
        "scrollable" => Scrollable,
        "space" => Space,
        "symbol" => Symbol,
        "function" => Function,
        "altfunction" => AltFunction,
        "unknown" => Unknown,
        "mouse" => Mouse,
        _ => return None,
    })
}

/// Registers and applies a bundled Nerd Font for both monospace and proportional rendering.
pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "NerdFont".to_owned(),
        FontData::from_static(include_bytes!(
            "../../assets/fonts/FiraCodeNerdFont-Regular.ttf"
        ))
        .into(),
    );
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "NerdFont".to_owned());
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "NerdFont".to_owned());
    ctx.set_fonts(fonts);
}
