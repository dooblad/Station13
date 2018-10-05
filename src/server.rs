#[macro_use]
extern crate enum_primitive;

pub mod net;

use std::io::Result;
use std::net::SocketAddrV4;

use net::common::*;
use net::common::socket::GameSocket;

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

fn main() -> Result<()> {
    use std::{thread, time};

    let mut server = Server::new(to_socket_addr(BIND_ADDR, SERVER_PORT));

    let ten_millis = time::Duration::from_millis(10);
    loop {
        server.tick();
        thread::sleep(ten_millis);
    }
}
