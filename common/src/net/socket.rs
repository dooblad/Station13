use std::fmt::Debug;
use std::io;
use std::io::Result;
use std::net::{SocketAddr, SocketAddrV4, UdpSocket};

use serde::{Deserialize, Serialize, UniqId};

use super::packet::*;
use super::*;

pub const PACKET_BUF_SIZE: usize = 4096;

pub struct GameSocket {
    socket: UdpSocket,
    packet_buf: [u8; PACKET_BUF_SIZE],
}

impl GameSocket {
    // TODO: Switch to IPv6 everything.
    pub fn new(bind_addr: SocketAddrV4) -> Self {
        let socket = UdpSocket::bind(bind_addr).expect(&format!("couldn't bind to {}", bind_addr));
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
            //result.push((Packet::deserialize(&self.packet_buf[..amt]), src));
        }
        result
    }

    pub fn send_to<S: Serialize + UniqId + Debug>(&mut self, data: S, dest: &SocketAddrV4) {
        //let bytes = {
        //    let mut bytes = vec![<S as UniqId>::id()];
        //    bytes.append(data.serialize());
        //    bytes
        //};
        //if bytes.len() > PACKET_BUF_SIZE {
        //    panic!(
        //        "serialized packet is too large ({} > {})",
        //        bytes.len(),
        //        PACKET_BUF_SIZE
        //    );
        //}
        //self.socket.send_to(bytes, dest).expect(&format!("failed to send {:?}", data));
    }
}
