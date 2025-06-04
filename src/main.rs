#![windows_subsystem = "windows"]

mod app;
mod config;
mod input;
mod platform;
mod ui;

fn main() {
    config::config::Config::ensure_config_exists().expect("Failed to write config");
    let tray_icon = ui::tray::spawn_tray();
    
    let target_os = std::env::var("CARGO_CFG_WINDOWS").is_ok();
    
    if target_os {
        embed_resource::compile("app.rc", embed_resource::NONE);
    }
    
    if let Err(err) = app::run() {
        #[cfg(debug_assertions)]
        eprintln!("Error: {:#?}", err);

        #[cfg(not(debug_assertions))]
        eprintln!("Something went wrong. Try restarting the app.");
        
        drop(tray_icon); // optional clean-up
        std::process::exit(1);
    }
}