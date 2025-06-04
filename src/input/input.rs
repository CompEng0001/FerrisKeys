use crate::input::{
    keyboard::resolve_physical_key,
    keymap::resolve_key_label,

};

use std::{
    thread,
    sync::{
        Arc, 
        atomic::{
            AtomicBool, 
            Ordering
        },
        mpsc::Sender
    },
};

use rdev::{
    listen, 
    EventType, 
    Key
};

#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress(String),
    MouseClick(String),
}

#[cfg(target_os = "windows")]
mod platform {
    use super::*;
    use crate::platform::windows::{translate_key_win32,vk_code_from_key};
    
    pub fn start_input_listener(tx: Sender<InputEvent>) {
        let shift_down = Arc::new(AtomicBool::new(false));
        let shift_flag = shift_down.clone();

        thread::spawn(move || {
            if let Err(err) = listen(move |event| {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        match key {
                            Key::ShiftLeft | Key::ShiftRight => {
                                shift_flag.store(true, Ordering::SeqCst);
                                tx.send(InputEvent::KeyPress("⇧ shift".into())).ok();
                            }
                            _ => {
                                let label = if shift_flag.load(Ordering::SeqCst) {
                                    vk_code_from_key(key)
                                        .and_then(|vk| translate_key_win32(vk))
                                        .unwrap_or_else(|| resolve_key_label(key))
                                } else {
                                    resolve_physical_key(key).unwrap_or_else(|| format!("{:?}", key))
                                };
                                tx.send(InputEvent::KeyPress(label)).ok();
                            }
                        }
                    }
                    EventType::KeyRelease(key) => {
                        if key == Key::ShiftLeft || key == Key::ShiftRight {
                            shift_flag.store(false, Ordering::SeqCst);
                        }
                    }
                    EventType::ButtonPress(button) => {

                        let label = format!("Mouse{:?}", button);
                        tx.send(InputEvent::MouseClick(label)).ok();
                    }
                    _ => {}
                }
            }) {
                eprintln!("Failed to listen to keyboard events: {:?}", err);
            }
        });
    }
}




#[cfg(target_os = "linux")]
mod platform {
    use super::*;
    use crate::linux_keyboard::resolve_linux_key;

    pub fn start_input_listener(tx: Sender<InputEvent>) {
        thread::spawn(move || {
            if let Err(err) = listen(move |event| {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        let raw = format!("{:?}", key);
                        println!("[INPUT] rdev key: {}", raw);
                        let label = resolve_linux_key(&raw).unwrap_or_else(|| raw.clone());
                        tx.send(InputEvent::KeyPress(label)).ok();
                    }
                    EventType::KeyRelease(key) => {
                        let raw = format!("{:?}", key);
                        println!("[RELEASE] rdev key: {}", raw);
                    }
                    EventType::ButtonPress(button) => {
                        let label = format!("Mouse{:?}", button);
                        tx.send(InputEvent::MouseClick(label)).ok();
                    }
                    _ => {}
                }
            }) {
                eprintln!("Failed to listen to keyboard events: {:?}", err);
            }
        });
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use super::*;
    use crate::macos_keyboard::resolve_macos_key;

    pub fn start_input_listener(tx: Sender<InputEvent>) {
        thread::spawn(move || {
            if let Err(err) = listen(move |event| {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        let raw = format!("{:?}", key);
                        println!("[INPUT] rdev key: {}", raw);
                        let label = resolve_macos_key(&raw).unwrap_or_else(|| raw.clone());
                        tx.send(InputEvent::KeyPress(label)).ok();
                    }
                    EventType::KeyRelease(key) => {
                        let raw = format!("{:?}", key);
                        println!("[RELEASE] rdev key: {}", raw);
                    }
                    EventType::ButtonPress(button) => {
                        let label = format!("Mouse{:?}", button);
                        tx.send(InputEvent::MouseClick(label)).ok();
                    }
                    _ => {}
                }
            }) {
                eprintln!("Failed to listen to keyboard events: {:?}", err);
            }
        });
    }
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub use platform::start_input_listener;
