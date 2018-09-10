use std::collections::HashSet;

use piston::input::*;
use piston::input::Button::{Keyboard, Mouse};
use piston::input::keyboard::Key;

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
                },
                // Mouse(m) => (),
                _ => (),
            }
        }

        if let Some(b) = event.release_args() {
            match b {
                Keyboard(k) => {
                    pressed_keys.remove(&k.code());
                },
                Mouse(m) => (),
                _ => (),
            }
        }
        /*
        let close_requested = &mut self.close_requested;
        let mouse_delta = &mut self.mouse_delta;
        let pressed_mouse_buttons = &mut self.pressed_mouse_buttons;
        // TODO: Make more efficient.
        self.last_pressed_keys = self.pressed_keys.clone();
        let pressed_keys = &mut self.pressed_keys;

        *mouse_delta = (0.0, 0.0);

        self.events_loop.poll_events(|event| {
            use glutin::Event::{Awakened, DeviceEvent, WindowEvent};

            match event {
                WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => *close_requested = true,
                    glutin::WindowEvent::KeyboardInput { input, .. } =>
                        Self::handle_key_event(input, pressed_keys, close_requested),
                    _ => (),
                },
                DeviceEvent { event, .. } => {
                    use glutin::DeviceEvent::*;
                    match event {
                        Added => (),
                        Removed => (),
                        Motion { axis, value, } => {
                            match axis {
                                0 => mouse_delta.0 += value,
                                1 => mouse_delta.1 += value,
                                _ => eprintln!("Unknown mouse axis {}", axis),
                            }
                        },
                        Button { button, state, } => {
                            match state {
                                glutin::ElementState::Pressed => {
                                    pressed_mouse_buttons.insert(button);
                                },
                                glutin::ElementState::Released => {
                                    pressed_mouse_buttons.remove(&button);
                                },
                            }
                        },
                        Key(input) => Self::handle_key_event(input, pressed_keys, close_requested),
                        Text { codepoint } => {
                            println!("Codepoint: {}", codepoint);
                        },
                    }
                },
                Awakened => (),
            }
        });
        */
    }

    /*
fn handle_key_event(input: glutin::KeyboardInput,
                    pressed_keys: &mut HashSet<VirtualKeyCode>,
                    close_requested: &mut bool) {
    match input {
        glutin::KeyboardInput { state, virtual_keycode: Some(key), modifiers, .. } => {
            match state {
                glutin::ElementState::Pressed => {
                    if modifiers.logo && key == VirtualKeyCode::W {
                        // [Command/Windows]-w was pressed, so we should close the window.
                        *close_requested = true;
                    }
                    pressed_keys.insert(key);
                },
                glutin::ElementState::Released => {
                    pressed_keys.remove(&key);
                },
            }
        },
        _ => (),
    };
    }
    */

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
        return self.pressed_keys.contains(&key.code()) &&
            !self.last_pressed_keys.contains(&key.code());
    }
}