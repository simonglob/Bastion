use crate::packets::{
    codec::{Decode, PacketID},
    intent::Intent,
    reader::Reader,
};
#[derive(Debug)]
pub struct Ping {
    pub timestamp: i64,
}

impl PacketID for Ping {
    fn id() -> u16 {
        0x01
    }
}

impl Decode for Ping {
    fn decode(reader: &mut Reader) -> std::io::Result<Self> {
        Ok(Self {
            timestamp: reader.read_i64(),
        })
    }
}
