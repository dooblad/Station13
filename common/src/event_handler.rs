use std::collections::HashSet;

use piston::input::keyboard::Key;
use piston::input::Button::{Keyboard, Mouse};
use piston::input::*;

#[derive(Clone)]
pub struct EventHandler {
    mouse_delta: (f64, f64),
    pressed_mouse_buttons: HashSet<u32>,
    // Store the currently-pressed keys and the keys that were pressed on the last tick.
    last_pressed_keys: HashSet<i32>,
    pressed_keys: HashSet<i32>,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            mouse_delta: (0.0, 0.0),
            pressed_mouse_buttons: HashSet::new(),
            last_pressed_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
        }
    }

    pub fn tick(&mut self, event: &Event) {
        // TODO: Make more efficient.
        self.last_pressed_keys = self.pressed_keys.clone();
        let pressed_keys = &mut self.pressed_keys;

        if let Some(b) = event.press_args() {
            match b {
                Keyboard(k) => {
                    pressed_keys.insert(k.code());
                }
                // Mouse(m) => (),
                _ => (),
            }
        }

        if let Some(b) = event.release_args() {
            match b {
                Keyboard(k) => {
                    pressed_keys.remove(&k.code());
                }
                Mouse(_) => (),
                _ => (),
            }
        }
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        return self.mouse_delta;
    }

    pub fn is_left_mouse_down(&self) -> bool {
        return self.pressed_mouse_buttons.contains(&1);
    }

    pub fn is_right_mouse_down(&self) -> bool {
        return self.pressed_mouse_buttons.contains(&3);
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        return self.pressed_keys.contains(&key.code());
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        return self.pressed_keys.contains(&key.code())
            && !self.last_pressed_keys.contains(&key.code());
    }
}
