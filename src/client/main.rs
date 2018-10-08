extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

#[macro_use]
extern crate game;

pub mod net;
pub mod render;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use game::ecs::Ecs;
use game::event_handler::EventHandler;
use game::net::*;

use net::Client;
use render::Renderer;

pub const WINDOW_TITLE: &'static str = "Station 13";
pub const WINDOW_DIMS: [u32; 2] = [800, 600];

pub const USERNAME: &'static str = "Doobs";

pub struct TickConfig {
    event_handler: EventHandler,
    dt: f64,
}

pub struct Game {
    gl: GlGraphics,
    client: Client,
    ecs: Ecs<TickConfig>,
    renderer: Renderer,
}

impl Game {
    pub fn new(gl: GlGraphics) -> Self {
        let mut client = Client::new(
            to_socket_addr(BIND_ADDR, CLIENT_PORT),
            to_socket_addr(BIND_ADDR, SERVER_PORT),
        );
        client.send(Packet::Hello {
            name: USERNAME.to_string(),
        });

        Game {
            gl,
            client,
            ecs: Ecs::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn tick(&mut self, tick_config: TickConfig) {
        self.client.tick();
        self.ecs.tick(&tick_config);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.renderer.render(&self.ecs, &mut self.gl, args);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(WINDOW_TITLE, WINDOW_DIMS)
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap();

    let mut event_handler = EventHandler::new();
    let mut game = Game::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        event_handler.tick(&e);

        if let Some(u) = e.update_args() {
            game.tick(TickConfig {
                // TODO: Cloning here is hella inefficient.
                event_handler: event_handler.clone(),
                dt: u.dt,
            });
        }

        if let Some(r) = e.render_args() {
            game.render(&r);
        }
    }
}
