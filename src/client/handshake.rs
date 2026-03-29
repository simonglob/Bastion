use crate::{network::state::ConnectionState, packets::{state::handshake::Handshake}};

pub fn handshake(packet: Handshake, state: &mut ConnectionState) -> std::io::Result<Vec<u8>> {
    state.update_state(packet.intent);
    
    Ok(vec![])
}