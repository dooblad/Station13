use std::io::Result;
use std::net::SocketAddrV4;

use game::ecs::Ecs;
use game::net::*;
use game::net::socket::GameSocket;

pub struct Server {
    socket: GameSocket,
    clients: Vec<SocketAddrV4>,
}

impl Server {
    pub fn new(bind_addr: SocketAddrV4) -> Self {
        Self {
            socket: GameSocket::new(bind_addr),
            clients: vec![],
        }
    }

    pub fn tick(&mut self) {
        for (packet, src) in self.socket.poll().iter() {
            match packet {
                Packet::Hello { name } => {
                    println!("player \"{}\" said hello", name);
                    self.clients.push(src.clone());
                    // TODO: Decide client ID.
                    self.socket.send_to(Packet::HelloAck {}, &src);
                },
                Packet::HelloAck {} => eprintln!("received HelloAck from client"),
            };
        }
    }

    pub fn socket(&self) -> &GameSocket { &self.socket }
}
