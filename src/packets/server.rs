use crate::packets::{
    codec::{Encode, PacketID},
    writer::Writer,
};

pub struct Handshake {
    pub json: String,
}

impl PacketID for Handshake {
    fn id() -> u16 {
        0x0
    }
}

impl Encode for Handshake {
    fn encode(&self) -> Vec<u8> {
        let mut writer = Writer::new();
        writer.write_str(&self.json);
        writer.get_content().to_vec()
    }
}
