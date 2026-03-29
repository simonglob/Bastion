use crate::packets::{
    codec::{Decode, PacketID},
    intent::Intent,
    reader::Reader,
};

#[allow(dead_code)]
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
    fn decode(reader: &mut Reader) -> std::io::Result<Self> {
        Ok(Self {
            protocol_version: reader.read_varint(),
            server_addr: reader.read_string(),
            server_port: reader.read_u16(),
            intent: Intent::from_value(reader.read_varint()),
        })
    }
}
