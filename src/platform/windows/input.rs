use crate::input::input::InputEvent;
use crate::input::{keyboard::resolve_physical_key, keymap::resolve_key_label};
use crate::platform::windows::layout::detect_layout;
use crate::platform::windows::windows::{translate_key_win32, vk_code_from_key};
use rdev::{listen, EventType, Key};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread,
};

/// Starts the input event listener loop for Windows, running in a background thread.
///
/// - Listens for key presses, key releases, and mouse button clicks.
/// - Uses the current keyboard layout to determine the correct symbol for a key.
/// - Handles Shift key detection manually to provide shifted character output.
/// - Translates virtual key codes into localized Unicode characters if Shift is active.
///
/// All events are sent via the provided `Sender<InputEvent>` to the main thread.
///
/// # Arguments
/// * `tx` - A `Sender<InputEvent>` used to transmit input events to the UI or processor.
pub fn start_input_listener(tx: Sender<InputEvent>) {
    let layout = detect_layout(); // Detect current keyboard layout once at startup

    let shift_down = Arc::new(AtomicBool::new(false)); // Track Shift key state
    let shift_flag = shift_down.clone(); // Clone for use in the event handler closure

    // Spawn the listener in its own thread so it doesn't block the main loop
    thread::spawn(move || {
        // Begin listening for input events
        if let Err(err) = listen(move |event| match event.event_type {
            // Handle key press events
            EventType::KeyPress(key) => match key {
                // Track when Shift is pressed
                Key::ShiftLeft | Key::ShiftRight => {
                    shift_flag.store(true, Ordering::SeqCst);
                    tx.send(InputEvent::KeyPress("⇧ shift".into())).ok();
                }

                // Other key presses
                _ => {
                    let label = if shift_flag.load(Ordering::SeqCst) {
                        // If Shift is active, try to resolve the actual shifted symbol
                        vk_code_from_key(key)
                            .and_then(|vk| translate_key_win32(vk)) // Try Win32 translation
                            .unwrap_or_else(|| resolve_key_label(key, &layout)) // Fallback to layout map
                    } else {
                        // Without Shift, resolve via physical map
                        resolve_physical_key(key)
                    };

                    tx.send(InputEvent::KeyPress(label)).ok();
                }
            },

            // Handle key release events
            EventType::KeyRelease(key) => {
                if key == Key::ShiftLeft || key == Key::ShiftRight {
                    shift_flag.store(false, Ordering::SeqCst);
                }
            }

            // Handle mouse button presses
            EventType::ButtonPress(button) => {
                let label = format!("Mouse{:?}", button);
                tx.send(InputEvent::MouseClick(label)).ok();
            }

            // Ignore other events
            _ => {}
        }) {
            eprintln!("Failed to listen to keyboard events: {:?}", err);
        }
    });
}
