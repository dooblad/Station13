extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

pub mod ecs;
pub mod event_handler;
pub mod level;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use event_handler::EventHandler;
use level::Level;

pub const WINDOW_TITLE: &'static str = "Station 13";
pub const WINDOW_DIMS: [u32; 2] = [800, 600];
pub const MOVE_SPEED: f64 = 500.0;

pub struct Game {
    gl: GlGraphics,
    level: Level,
    pos: (f64, f64),
}

impl Game {
    pub fn new(gl: GlGraphics) -> Self {
        Game {
            gl,
            level: Level::new(),
            pos: (0.0, 0.0),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let pos = self.pos;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                .trans(-pos.0, -pos.1);

            rectangle(RED, square, transform, gl);
        });
    }

    fn tick(&mut self, args: &UpdateArgs, event_handler: &EventHandler) {
        let ms_dt = MOVE_SPEED * args.dt;
        if event_handler.is_key_down(Key::W) {
            self.pos.1 += ms_dt;
        }
        if event_handler.is_key_down(Key::S) {
            self.pos.1 -= ms_dt;
        }
        if event_handler.is_key_down(Key::A) {
            self.pos.0 += ms_dt;
        }
        if event_handler.is_key_down(Key::D) {
            self.pos.0 -= ms_dt;
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(WINDOW_TITLE, WINDOW_DIMS)
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap();

    let mut game = Game::new(GlGraphics::new(opengl));
    let mut event_handler = EventHandler::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        event_handler.tick(&e);

        if let Some(u) = e.update_args() {
            game.tick(&u, &event_handler);
        }

        if let Some(r) = e.render_args() {
            game.render(&r);
        }
    }
}
