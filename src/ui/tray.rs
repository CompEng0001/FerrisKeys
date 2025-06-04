use tray_icon::{
    menu::{Menu, MenuItem, MenuEvent},
    TrayIconBuilder, TrayIcon
};
use std::{path::PathBuf, process::Command, thread};

#[cfg(target_os = "windows")]
fn load_embedded_icon() -> tray_icon::Icon {
    tray_icon::Icon::from_resource(1, None).expect("Failed to load embedded icon")
}

fn get_config_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        dirs::data_dir().unwrap().join("ferriskeys")
    } else {
        dirs::config_dir().unwrap().join("ferriskeys")
    }
}

fn open_config_folder() {
    let config_dir = get_config_path();
    let _ = if cfg!(target_os = "windows") {
        Command::new("explorer").arg(config_dir).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(config_dir).spawn()
    } else {
        Command::new("xdg-open").arg(config_dir).spawn()
    };
}

pub fn spawn_tray() -> TrayIcon {
    let open_item = MenuItem::new("Open Config", true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    let open_id = open_item.id().clone();
    let quit_id = quit_item.id().clone();

    let menu = Menu::new();
    menu.append(&open_item).unwrap();
    menu.append(&quit_item).unwrap();

    let tray_icon = TrayIconBuilder::new()
        .with_icon(load_embedded_icon()) 
        .with_menu(Box::new(menu))
        .with_tooltip("FerrisKeys")
        .build()
        .expect("Could not create tray icon");

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

    tray_icon
}
