use bytes::{Buf, Bytes};

use crate::packets::{
    codec::{Decode, PacketID},
    intent::Intent,
    reader::MinecraftProtocol,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_addr: String,
    pub server_port: u16,
    pub intent: Intent,
}

impl PacketID for Handshake {
    fn id() -> u16 {
        0x0
    }
}

impl Decode for Handshake {
    fn decode(reader: &mut Bytes) -> std::io::Result<Self> {
        Ok(Self {
            protocol_version: reader.get_var_int(),
            server_addr: reader.get_string(),
            server_port: reader.get_u16(),
            intent: Intent::from_value(reader.get_var_int()),
        })
    }
}
