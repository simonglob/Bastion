use crate::packets::{
    codec::{Decode, Encode, PacketID},
    reader::Reader,
    writer::Writer,
};

#[derive(Debug)]
pub struct StatusRequest {}

impl PacketID for StatusRequest {
    fn id() -> u16 {
        0x00
    }
}

impl Decode for StatusRequest {
    fn decode(_: &mut Reader) -> std::io::Result<Self> {
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
    fn encode(&self) -> Vec<u8> {
        let mut writer = Writer::new();
        writer.write_str(&self.json);
        writer.get_content().to_vec()
    }
}

// simple echo
pub struct PingPong {
    pub timestamp: i64,
}

impl PacketID for PingPong {
    fn id() -> u16 {
        0x01
    }
}

impl Encode for PingPong {
    fn encode(&self) -> Vec<u8> {
        let mut writer = Writer::new();
        writer.write_int(self.timestamp);
        writer.get_content().to_vec()
    }
}

impl Decode for PingPong {
    fn decode(reader: &mut Reader) -> std::io::Result<Self> {
        Ok(Self {
            timestamp: reader.read_i64(),
        })
    }
}
