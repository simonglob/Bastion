use crate::{network::session::Session, packets::state::handshake::Handshake};

pub async fn handshake(packet: Handshake, state: &mut Session) -> std::io::Result<Vec<u8>> {
    state.update_state(packet.intent);

    Ok(vec![])
}
