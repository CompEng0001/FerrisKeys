use crate::input::input::InputEvent;
use crate::input::{keyboard::resolve_physical_key, keymap::resolve_key_label};
use crate::platform::linux::layout::detect_layout;
use rdev::{listen, EventType, Key};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread,
};

/// Starts the Linux input event listener in a background thread.
///
/// Listens to global key and mouse events using `rdev::listen()`, then:
/// - Resolves key labels based on the current keyboard layout.
/// - Tracks the Shift key status manually to support shifted characters.
/// - Sends processed input events (keyboard or mouse) to the main application
///   via the given `Sender<InputEvent>`.
///
/// This listener is Linux-specific and handles layout-aware translation without Win32 APIs.
///
/// # Arguments
/// * `tx` - A channel `Sender` to push `InputEvent` messages to the application.
pub fn start_input_listener(tx: Sender<InputEvent>) {
    let layout = detect_layout(); // Detect the active keyboard layout once at startup

    let shift_down = Arc::new(AtomicBool::new(false)); // Shared state to track Shift press
    let shift_flag = shift_down.clone(); // Clone for use inside event handler

    thread::spawn(move || {
        if let Err(err) = listen(move |event| match event.event_type {
            // Handle key press
            EventType::KeyPress(key) => {
                if key == Key::ShiftLeft || key == Key::ShiftRight {
                    shift_flag.store(true, Ordering::SeqCst);
                    tx.send(InputEvent::KeyPress("⇧ shift".into())).ok();
                } else {
                    // Resolve label based on shift state and layout
                    let label = if shift_flag.load(Ordering::SeqCst) {
                        resolve_key_label(key, &layout)
                    } else {
                        resolve_physical_key(key)
                    };

                    tx.send(InputEvent::KeyPress(label)).ok();
                }
            }

            // Handle key release
            EventType::KeyRelease(key) => {
                if key == Key::ShiftLeft || key == Key::ShiftRight {
                    shift_flag.store(false, Ordering::SeqCst);
                } else {
                    let _raw = format!("{:?}", key);
                    // Debug logging can be inserted here if needed
                }
            }

            // Handle mouse button press
            EventType::ButtonPress(button) => {
                let label = format!("Mouse{:?}", button);
                tx.send(InputEvent::MouseClick(label)).ok();
            }

            // Ignore other events (e.g., mouse move, scroll, etc.)
            _ => {}
        }) {
            eprintln!("Failed to listen to keyboard events: {:?}", err);
        }
    });
}
