#[macro_use]
extern crate enum_primitive;

pub mod common;

use std::io;
use std::io::Result;
use std::net::{UdpSocket, SocketAddrV4};

use self::common::*;

pub struct Client {
    pub socket: UdpSocket,
    pub server_addr: SocketAddrV4,
    packet_buf: [u8; PACKET_BUF_SIZE],
}

impl Client {
    pub fn new(client_addr: SocketAddrV4, server_addr: SocketAddrV4) -> Self {
        // TODO: Dedup between client and server (make `GameSocket` struct?).
        let socket = UdpSocket::bind(client_addr).expect(
            &format!("couldn't bind to {}", client_addr));
        // We don't want to wait indefinitely for incoming requests.  Rather, we want to peek
        // during every tick.
        socket.set_nonblocking(true).unwrap();

        Self {
            socket,
            server_addr,
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

            let (amt, src) = packet_info;
            let packet = Packet::deserialize(&self.packet_buf[..amt]);
            println!("received {:?}", packet);
        }
    }

    pub fn send<S: Serialize>(&mut self, data: S) -> Result<()> {
        self.send_bytes(data.serialize())
    }

    fn send_bytes(&mut self, data: Vec<u8>) -> Result<()> {
        self.socket.send_to(&data, self.server_addr)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    use std::{thread, time};

    let mut client = Client::new(
        to_socket_addr(BIND_ADDR, CLIENT_PORT),
        to_socket_addr(BIND_ADDR, SERVER_PORT));
    client.send(Packet::Hello { name: "doobs".to_string() })?;

    let ten_millis = time::Duration::from_millis(10);
    loop {
        client.tick();
        thread::sleep(ten_millis);
    }
}
