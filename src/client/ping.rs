use crate::{
    network::state::ConnectionState,
    packets::{codec::write_packet, state::status::PingPong},
};

// echo packet
pub fn ping(packet: PingPong, _: &mut ConnectionState) -> std::io::Result<Vec<u8>> {
    Ok(write_packet(&PingPong {
        timestamp: packet.timestamp,
    }))
}
