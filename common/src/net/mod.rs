pub mod socket;

use std;
use std::net::{Ipv4Addr, SocketAddrV4};

use enum_primitive::FromPrimitive;

use super::alloc::GenerationalIndex;
use super::ecs::Entity;

pub const PACKET_BUF_SIZE: usize = 4096;

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
            panic!(
                "serialized data is too large ({} > {})",
                result.len(),
                PACKET_BUF_SIZE
            );
        }
        result
    }
}

pub trait Deserialize {
    fn deserialize(data: &[u8]) -> Self;
}

// TODO: Make this an enum of enums (for client-only, server-only, and common packets)?
// Or maybe they should be entirely disjoint...
#[derive(Debug)]
pub enum Packet {
    Hello {
        name: String,
    },
    HelloAck,
    CreateEntity {
        entity: Entity,
    },
    SetComponent {
        entity: Entity,
        comp_id: u32,
        data: Vec<u8>,
    },
}

enum_from_primitive! {
pub enum PacketId {
    Hello = 0,
    HelloAck = 1,
    CreateEntity = 2,
    SetComponent = 3,
}
}

impl Packet {
    pub fn id(&self) -> u8 {
        use self::Packet::*;
        match *self {
            Hello { .. } => PacketId::Hello as u8,
            HelloAck { .. } => PacketId::HelloAck as u8,
            CreateEntity { .. } => PacketId::CreateEntity as u8,
            SetComponent { .. } => PacketId::SetComponent as u8,
        }
        // TODO: Open an issue for why we can't cast the result of the entire match to a `u8`.
    }
}

impl Serialize for Packet {
    fn raw_data(&self) -> Vec<u8> {
        use self::Packet::*;
        let mut result = vec![self.id()];
        match *self {
            Hello { ref name } => result.append(&mut name.clone().into_bytes()),
            HelloAck { .. } => (),
            CreateEntity { ref entity } => {
                result.append(&mut (entity.idx as u64).raw_data());
                result.append(&mut (entity.gen as u64).raw_data());
            }
            SetComponent { .. } => unimplemented!(),
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
            }
            PacketId::HelloAck => HelloAck {},
            PacketId::CreateEntity => {
                let idx = <u64>::deserialize(&rest[0..8]) as usize;
                let gen = <u64>::deserialize(&rest[8..16]);
                println!("idx: {}, gen: {}", idx, gen);
                CreateEntity {
                    entity: GenerationalIndex { idx, gen },
                }
            }
            PacketId::SetComponent => unimplemented!(),
        }
    }
}

impl Serialize for u64 {
    fn raw_data(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(8);
        for shift in (0..8).map(|v| v * 8).rev() {
            result.push(((*self >> shift) & 0xff) as u8);
        }
        result
    }
}

impl Deserialize for u64 {
    fn deserialize(data: &[u8]) -> Self {
        let mut result = 0u64;
        let shift_iter = (0..8).map(|v| v * 8).rev();
        for (byte, shift) in data.iter().zip(shift_iter) {
            result += (*byte as u64) << shift;
        }
        result
    }
}
