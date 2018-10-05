use std::fmt::Debug;
use std::io;
use std::io::Result;
use std::net::{UdpSocket, SocketAddr, SocketAddrV4};

use super::*;

pub struct GameSocket {
    socket: UdpSocket,
    packet_buf: [u8; PACKET_BUF_SIZE],
}

impl GameSocket {
    // TODO: Switch to IPv6 everything.
    pub fn new(bind_addr: SocketAddrV4) -> Self {
        let socket = UdpSocket::bind(bind_addr).expect(
            &format!("couldn't bind to {}", bind_addr));
        // We don't want to wait indefinitely for incoming requests.  Rather, we want to peek
        // during every tick.
        socket.set_nonblocking(true).unwrap();

        Self {
            socket,
            packet_buf: [0; PACKET_BUF_SIZE],
        }
    }

    /// Drains the queue of incoming packets on this socket and returns them in the order they were
    /// received.
    pub fn poll(&mut self) -> Vec<(Packet, SocketAddrV4)> {
        let mut result: Vec<(Packet, SocketAddrV4)> = vec![];
        loop {
            let packet_info = match self.socket.recv_from(&mut self.packet_buf) {
                Ok(p) => p,
                // Once we've grabbed everything, break out of here.
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => panic!("encountered IO error: {}", e),
            };
            let (amt, src) = packet_info;
            let src = match src {
                SocketAddr::V4(s) => s,
                SocketAddr::V6(_) => panic!("IPv6 currently unsupported"),
            };
            result.push((Packet::deserialize(&self.packet_buf[..amt]), src));
        }
        result
    }

    pub fn send_to<S: Serialize + Debug>(&mut self, data: S, dest: &SocketAddrV4) {
        self.send_bytes_to(data.serialize(), dest)
            .expect(&format!("failed to send {:?}", data));
    }

    fn send_bytes_to(&mut self, data: Vec<u8>, dest: &SocketAddrV4) -> Result<()> {
        self.socket.send_to(&data, dest)?;
        Ok(())
    }
}