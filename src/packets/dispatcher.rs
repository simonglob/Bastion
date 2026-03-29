use std::collections::HashMap;

use crate::packets::{codec::Decode, reader::Reader};

type BoxedHandler = Box<dyn Fn(&mut Reader) -> std::io::Result<Vec<u8>> + Send + Sync>;

#[derive(Default)]
pub struct PacketDispatcher {
    handlers: HashMap<u16, BoxedHandler>
}

impl PacketDispatcher {
    pub fn register<P, F>(&mut self, handler: F) 
    where
        P: Decode + 'static,
        F: Fn(P) -> std::io::Result<Vec<u8>> + Send + Sync + 'static,
    {
        self.handlers.insert(P::id(), Box::new(move |mut reader| {
            println!("parsing packet {:?} with buffer {:?}", P::id(), reader.get_buffer());
            let decoded = P::decode(&mut reader)?;
            handler(decoded)
        }));
    }

    pub fn dispatch(&self, id: u16, payload: &mut Reader) -> std::io::Result<Vec<u8>> {
        match self.handlers.get(&id) {
            Some(h) => h(payload),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("unhandled or unknown packet id {id:#04x}")
            ))
        }
    }
}