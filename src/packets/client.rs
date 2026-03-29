use crate::packets::{codec::{Decode, PacketID}, reader::Reader};

#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_addr: String,
    pub server_port: u16,
    pub intent: i32
}

impl PacketID for Handshake { 
    fn id() -> u16 { 0x0 } 
}

impl Decode for Handshake {
    fn decode(reader: &mut Reader) -> std::io::Result<Self> {
        Ok(Self {
            protocol_version: reader.read_varint(),
            server_addr: reader.read_string(),
            server_port: reader.read_u16(),
            intent: reader.read_varint(),
        })
    }
}

#[derive(Debug)]
pub struct Ping {
    pub timestamp: i64
}

impl PacketID for Ping {
    fn id() -> u16 { 0x01 }
}

impl Decode for Ping {
    fn decode(reader: &mut Reader) -> std::io::Result<Self> {
        Ok(Self {
            timestamp: reader.read_i64()
        })
    }
}