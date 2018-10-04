#[macro_use]
extern crate enum_primitive;

pub mod common;

use std::fmt::Debug;
use std::io;
use std::io::Result;
use std::net::{UdpSocket, SocketAddr, SocketAddrV4};

use self::common::*;

pub struct Server {
    pub socket: UdpSocket,
    packet_buf: [u8; PACKET_BUF_SIZE],
}

impl Server {
    pub fn new(addr: SocketAddrV4) -> Self {
        let socket = UdpSocket::bind(addr).expect(&format!("couldn't bind to {}", addr));
        // We don't want to wait indefinitely for incoming requests.  Rather, we want to peek
        // during every tick.
        socket.set_nonblocking(true).unwrap();

        Self {
            socket,
            packet_buf: [0; PACKET_BUF_SIZE],
        }
    }

    pub fn tick(&mut self) {
        loop {
            let packet_info = match self.socket.recv_from(&mut self.packet_buf) {
                Ok(p) => p,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return,
                Err(e) => panic!("encountered IO error: {}", e),
            };

            let (amt, client) = packet_info;
            let packet = Packet::deserialize(&self.packet_buf[..amt]);
            println!("received {:?}", packet);
            match packet {
                Packet::Hello { name } => {
                    println!("client \"{}\" said hello", name);
                    self.send(Packet::HelloAck {}, &client);
                },
                Packet::HelloAck {} => eprintln!("received HelloAck from client"),
            };
        }
    }

    pub fn send<S: Serialize + Debug>(&mut self, data: S, client: &SocketAddr) {
        self.send_bytes(data.serialize(), client)
            .expect(&format!("failed to send {:?}", data));
    }

    fn send_bytes(&mut self, data: Vec<u8>, client: &SocketAddr) -> Result<()> {
        println!("sending {:?}", data);
        self.socket.send_to(&data, client)?;
        Ok(())
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
