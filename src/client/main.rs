extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate game;

pub mod net;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use game::event_handler::EventHandler;
use game::ecs::Ecs;
use game::net::*;

use net::Client;

pub const WINDOW_TITLE: &'static str = "Station 13";
pub const WINDOW_DIMS: [u32; 2] = [800, 600];

pub const USERNAME: &'static str = "Doobs";

pub struct Game {
    gl: GlGraphics,
    client: Client,
    ecs: Ecs,
}

impl Game {
    pub fn new(gl: GlGraphics) -> Self {
        let mut client = Client::new(
            to_socket_addr(BIND_ADDR, CLIENT_PORT),
            to_socket_addr(BIND_ADDR, SERVER_PORT));
        client.send(Packet::Hello { name: USERNAME.to_string() });

        Game {
            gl,
            client,
            ecs: Ecs::new(),
        }
    }

    pub fn tick(&mut self, args: &UpdateArgs, event_handler: &EventHandler) {
        self.client.tick();
        self.ecs.tick(args, event_handler);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.ecs.render(&mut self.gl, args);
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
