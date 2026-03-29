use crate::packets::{reader::Reader, writer::Writer};

pub trait PacketID {
    fn id() -> u16;
}

pub trait Encode: PacketID {
    fn encode(&self) -> Vec<u8>;
}

pub trait Decode: PacketID + Sized {
    fn decode(reader: &mut Reader) -> std::io::Result<Self>;
}

pub fn write_packet<P: Encode>(packet: &P) -> Vec<u8> {
    let payload = packet.encode();
    let length = 6 + payload.len(); // 6 because 4 for length and 2 for packet id

    let mut writer = Writer::new();
    writer.write_varint(length as u32);
    writer.write_varint(P::id() as u32);
    writer.write_all(&payload);

    writer.get_content().to_vec()
}

// returns (length, packet_id, payload_start_pos)
pub fn read_frame(reader: &mut Reader) -> Option<(usize, u16)> {
    if reader.is_empty() {
        return None;
    }

    // length is the length of packet id + data
    let length = reader.read_varint();
    if length < 0 {
        return None;
    }

    let packet_id = reader.read_varint() as u16;

    Some((length as usize, packet_id))
}