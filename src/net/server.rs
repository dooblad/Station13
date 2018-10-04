#[macro_use]
extern crate enum_primitive;

pub mod common;

use std::io::Result;
use std::net::SocketAddrV4;

use self::common::*;
use self::socket::GameSocket;

pub struct Server {
    pub socket: GameSocket,
}

impl Server {
    pub fn new(bind_addr: SocketAddrV4) -> Self {
        Self {
            socket: GameSocket::new(bind_addr),
        }
    }

    pub fn tick(&mut self) {
        for (packet, src) in self.socket.poll().iter() {
            match packet {
                Packet::Hello { name } => {
                    println!("client \"{}\" said hello", name);
                    self.socket.send_to(Packet::HelloAck {}, src);
                },
                Packet::HelloAck {} => eprintln!("received HelloAck from client"),
            };
        }
    }
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
