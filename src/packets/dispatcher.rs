use std::collections::HashMap;

use crate::{
    network::state::ConnectionState,
    packets::{codec::Decode, intent::Intent, reader::Reader},
};

type BoxedHandler =
    Box<dyn Fn(&mut Reader, &mut ConnectionState) -> std::io::Result<Vec<u8>> + Send + Sync>;

#[derive(Default)]
pub struct PacketDispatcher {
    handlers: HashMap<(u16, Intent), BoxedHandler>,
}

impl PacketDispatcher {
    pub fn register<P, F>(&mut self, handler: F, intent: Intent)
    where
        P: Decode + 'static,
        F: Fn(P, &mut ConnectionState) -> std::io::Result<Vec<u8>> + Send + Sync + 'static,
    {
        self.handlers.insert(
            (P::id(), intent),
            Box::new(move |mut reader, connection_state| {
                let decoded = P::decode(&mut reader)?;
                handler(decoded, connection_state)
            }),
        );
    }

    pub fn dispatch(
        &self,
        id: u16,
        payload: &mut Reader,
        state: &mut ConnectionState,
    ) -> std::io::Result<Vec<u8>> {
        match self.handlers.get(&(id, state.get_state())) {
            Some(handler) => handler(payload, state),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("unhandled or unknown packet id {id:#04x}"),
            )),
        }
    }
}
