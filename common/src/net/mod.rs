pub mod socket;

use std;
use std::net::{Ipv4Addr, SocketAddrV4};

use enum_primitive::FromPrimitive;

use serde::{Deserialize, Serialize};

use crate::alloc::GenerationalIndex;
use crate::ecs::Entity;

// TODO: Once const functions are in stable, we should make these SocketAddrV4's.
pub const BIND_ADDR: [u8; 4] = [127, 0, 0, 1];
pub const SERVER_PORT: u16 = 7878;
pub const CLIENT_PORT: u16 = 7777;

pub fn to_socket_addr(addr: [u8; 4], port: u16) -> SocketAddrV4 {
    SocketAddrV4::new(Ipv4Addr::from(addr), port)
}

// TODO: Make this an enum of enums (for client-only, server-only, and common packets)?
// Or maybe they should be entirely disjoint...
mod packet {
    use crate::ecs::Entity;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serde)]
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
}
