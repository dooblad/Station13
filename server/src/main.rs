#[macro_use]
extern crate common;

pub mod net;
pub mod random_mob;

use std::net::SocketAddrV4;

use common::ecs::alloc::GenerationalIndex;
use common::ecs::Ecs;
use common::net::socket::GameSocket;
use common::net::packet::Packet;
use common::net::*;

use self::random_mob::RandomMobUpdateSystem;

pub struct TickConfig {
    dt: f64,
}

pub struct Game {
    ecs: Ecs<TickConfig>,
    socket: GameSocket,
    clients: Vec<SocketAddrV4>,
}

impl Game {
    pub fn new() -> Self {
        let mut result = Self {
            ecs: Ecs::new(),
            socket: GameSocket::new(to_socket_addr(BIND_ADDR, SERVER_PORT)),
            clients: vec![],
        };
        result
            .ecs
            .systems()
            .append(&mut sys_vec![RandomMobUpdateSystem]);
        let _ = random_mob::new(&mut result.ecs);
        result
    }

    pub fn tick(&mut self) {
        // TODO: Should the logic tick and the network tick be ran in the same order as on the
        // client?
        for (packet, src) in self.socket.poll().iter() {
            match packet {
                Packet::Hello { name } => {
                    println!("player \"{}\" said hello", name);
                    self.clients.push(src.clone());
                    // TODO: Decide client ID.
                    self.socket.send_to(Packet::HelloAck {}, &src);
                    for entity in self.ecs.entities() {
                        // TODO: Send the mob's info to the client.
                        self.socket.send_to(Packet::CreateEntity { entity: entity.clone() }, &src);
                        //let comp_map = self.ecs.entity_map.borrow(&entity).unwrap();
                        //comp_map.get(PositionComponent { x: 0.0, y: 0.0 });
                        //comp_map.get(PlayerComponent { control_scheme });
                        //comp_map.get(RenderComponent {
                    }
                }
                Packet::HelloAck { .. }
                | Packet::CreateEntity { .. }
                | Packet::SetComponent { .. } => {
                    eprintln!("received invalid packet from client: {:?}", packet)
                }
            };
        }

        // TODO: Add *real* time deltas.
        let tick_config = TickConfig { dt: 1.0 };
        self.ecs.tick(&tick_config);
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
