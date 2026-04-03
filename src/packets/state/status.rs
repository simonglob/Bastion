use bytes::{Buf, Bytes};

use crate::packets::{
    codec::{Decode, Encode, PacketID},
    writer::Writer,
};
// simple echo

#[derive(Debug)]
pub struct StatusRequest {}

impl PacketID for StatusRequest {
    fn id() -> u16 {
        0x00
    }
}

impl Decode for StatusRequest {
    fn decode(_: &mut Bytes) -> std::io::Result<Self> {
        Ok(Self {})
    }
}

pub struct StatusResponse {
    pub json: String,
}

impl PacketID for StatusResponse {
    fn id() -> u16 {
        0x00
    }
}

impl Encode for StatusResponse {
    fn encode(&self, writer: &mut Writer) {
        writer.write_str(&self.json);
    }
}

#[derive(Debug)]
pub struct PingPong {
    pub timestamp: i64,
}

impl PacketID for PingPong {
    fn id() -> u16 {
        0x01
    }
}

impl Encode for PingPong {
    fn encode(&self, writer: &mut Writer) {
        writer.write(&self.timestamp.to_be_bytes());
    }
}

impl Decode for PingPong {
    fn decode(buffer: &mut Bytes) -> std::io::Result<Self> {
        Ok(Self {
            timestamp: buffer.get_i64(),
        })
    }
}
