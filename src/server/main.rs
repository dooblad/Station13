#[macro_use]
extern crate game;

pub mod net;
pub mod random_mob;

use game::ecs::Ecs;
use game::net::*;

use net::Server;

// TODO: Add time deltas.
pub struct TickConfig {
    dt: f64,
}

pub struct Game {
    ecs: Ecs<TickConfig>,
    server: Server,
}

impl Game {
    pub fn new() -> Self {
        let mut result = Self {
            ecs: Ecs::new(),
            server: Server::new(to_socket_addr(BIND_ADDR, SERVER_PORT)),
        };
        //result.systems().append(sys_vec![RandomMobUpdateSystem]);
        result
    }

    pub fn tick(&mut self) {
        // TODO: Should the logic tick and the network tick be ran in the same order as on the
        // client?
        let tick_config = TickConfig { dt: 1.0 };
        self.ecs.tick(&tick_config);
        self.server.tick();
    }
}

fn main() {
    use std::{thread, time};

    let mut game = Game::new();
    let ten_millis = time::Duration::from_millis(10);

    loop {
        game.tick();
        thread::sleep(ten_millis);
    }
}
