pub mod socket;

use std;
use std::net::{Ipv4Addr, SocketAddrV4};

use enum_primitive::FromPrimitive;

pub const PACKET_BUF_SIZE: usize = 1024;

// TODO: Once const functions are in stable, we should make these a SocketAddrV4.
pub const BIND_ADDR: [u8; 4] = [127, 0, 0, 1];
pub const SERVER_PORT: u16 = 7878;
pub const CLIENT_PORT: u16 = 7777;

pub fn to_socket_addr(addr: [u8; 4], port: u16) -> SocketAddrV4 {
    SocketAddrV4::new(Ipv4Addr::from(addr), port)
}

pub trait Serialize {
    // This method should only be implemented by types that are serializable, but never used.
    // `serialize` is the only method that should be used.
    fn raw_data(&self) -> Vec<u8>;
    // Serializes the type and ensures the resulting bytes are small enough to fit within a packet.
    fn serialize(&self) -> Vec<u8> {
        let result = self.raw_data();
        if result.len() > PACKET_BUF_SIZE {
            panic!("serialized data is too large ({} > {})", result.len(), PACKET_BUF_SIZE);
        }
        result
    }
}

pub trait Deserialize {
    fn deserialize(data: &[u8]) -> Self;
}

#[derive(Debug)]
pub enum Packet {
    Hello { name: String },
    HelloAck,
}

enum_from_primitive! {
pub enum PacketId {
    Hello = 0,
    HelloAck = 1,
}
}

impl Packet {
    pub fn id(&self) -> u8 {
        use self::Packet::*;
        match *self {
            Hello { name: _ } => PacketId::Hello as u8,
            HelloAck {} => PacketId::HelloAck as u8,
        }
        // TODO: Open an issue for why we can't cast the result of the entire match to a `u8`.
    }
}

impl Serialize for Packet {
    fn raw_data(&self) -> Vec<u8> {
        use self::Packet::*;
        let mut result = vec![self.id()];
        match *self {
            Hello {ref name} => {
                result.append(&mut name.clone().into_bytes())
            },
            HelloAck {} => (),
        }
        result
    }
}

impl Deserialize for Packet {
    fn deserialize(data: &[u8]) -> Self {
        use self::Packet::*;
        let (id, rest) = data.split_at(1);
        let id_enum = PacketId::from_u8(id[0]).expect(&format!("unknown packet ID {}", id[0]));
        match id_enum {
            PacketId::Hello => {
                let name = std::str::from_utf8(rest)
                    .expect("couldn't decode UTF-8 string")
                    .to_string();
                Hello { name }
            },
            PacketId::HelloAck => {
                HelloAck {}
            },
        }
    }
}
