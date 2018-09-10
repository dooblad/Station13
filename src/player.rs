use std::collections::HashMap;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::input::*;

use event_handler::EventHandler;

pub const MOVE_SPEED: f64 = 500.0;

#[derive(PartialEq, Eq, Hash)]
pub enum Intent {
    Up,
    Down,
    Left,
    Right,
}
pub struct ControlScheme(pub HashMap<Intent, Key>);
impl ControlScheme {
    pub fn new() -> Self {
        ControlScheme(HashMap::new())
    }

    pub fn intends(&self, intent: Intent, event_handler: &EventHandler) -> bool {
        if let Some(&k) = self.0.get(&intent) {
            event_handler.is_key_down(k)
        } else {
            false
        }
    }
}

pub struct Player {
    pos: (f64, f64),
    control_scheme: ControlScheme,
}

impl Player {
    pub fn new(control_scheme: ControlScheme) -> Self {
        Self {
            pos: (0.0, 0.0),
            control_scheme,
        }
    }

    pub fn tick(&mut self, args: &UpdateArgs, event_handler: &EventHandler) {
        use self::Intent::*;

        let ms_dt = MOVE_SPEED * args.dt;
        if self.control_scheme.intends(Up, event_handler) {
            self.pos.1 += ms_dt;
        }
        if self.control_scheme.intends(Down, event_handler) {
            self.pos.1 -= ms_dt;
        }
        if self.control_scheme.intends(Left, event_handler) {
            self.pos.0 += ms_dt;
        }
        if self.control_scheme.intends(Right, event_handler) {
            self.pos.0 -= ms_dt;
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, c: Context, args: &RenderArgs) {
        use graphics::*;
        const RED: [f32; 4] = [0.7, 0.3, 0.3, 1.0];
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let square = rectangle::square(0.0, 0.0, 50.0);
        let transform = c.transform.trans(x, y)
            .trans(-self.pos.0, -self.pos.1);
        rectangle(RED, square, transform, gl);
    }
}