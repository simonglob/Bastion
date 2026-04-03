use bytes::{Buf, Bytes};

use crate::packets::{reader::MinecraftProtocol, writer::Writer};

pub trait PacketID {
    fn id() -> u16;
}

pub trait Encode {
    fn encode(&self, writer: &mut Writer);
}

pub trait Decode: PacketID + Sized {
    fn decode(buffer: &mut Bytes) -> std::io::Result<Self>;
}

pub fn write_packet<P: PacketID + Encode>(packet: &P) -> Vec<u8> {
    let mut inner = Writer::new();
    inner.write_varint(P::id() as i32);
    packet.encode(&mut inner);

    let inner_bytes = inner.get_content();

    let mut writer = Writer::new();
    writer.write_varint(inner_bytes.len() as i32);
    writer.write(inner_bytes);

    writer.get_content().to_vec()
}

// returns (length, packet_id, payload_start_pos)
pub fn read_frame(buffer: &mut Bytes) -> Option<(usize, u16)> {
    if !buffer.has_remaining() {
        return None;
    }

    // length is the length of packet id + data
    let length = buffer.get_var_int();
    if length < 0 {
        return None;
    }

    let packet_id = buffer.get_var_int() as u16;

    Some((length as usize, packet_id))
}
