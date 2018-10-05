use std::fmt::Debug;
use std::io::Result;
use std::net::SocketAddrV4;

use game::net::*;
use game::net::socket::GameSocket;

pub struct Client {
    pub socket: GameSocket,
    pub server_addr: SocketAddrV4,
}

impl Client {
    pub fn new(bind_addr: SocketAddrV4, server_addr: SocketAddrV4) -> Self {
        Self {
            socket: GameSocket::new(bind_addr),
            server_addr,
        }
    }

    pub fn tick(&mut self) {
        for (packet, _) in self.socket.poll().iter() {
            match packet {
                Packet::Hello { name: _ } => eprintln!("received Hello from server"),
                Packet::HelloAck {} => {
                    println!("received {:?}", packet);
                },
            };
        }
    }

    pub fn send<S: Serialize + Debug>(&mut self, data: S) {
        self.socket.send_to(data, &self.server_addr);
    }
}
