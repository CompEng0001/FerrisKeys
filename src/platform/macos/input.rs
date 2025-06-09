#[cfg(target_os = "macos")]
mod platform {
    use super::*;
    use crate::macos_keyboard::resolve_macos_key;

    pub fn start_input_listener(tx: Sender<InputEvent>) {
        thread::spawn(move || {
            if let Err(err) = listen(move |event| match event.event_type {
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
            }) {
                eprintln!("Failed to listen to keyboard events: {:?}", err);
            }
        });
    }
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub use platform::start_input_listener;