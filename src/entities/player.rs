use uuid::Uuid;

use crate::{
    entities::property::Properties,
    packets::{codec::Encode, writer::Writer},
};

#[derive(Debug)]
pub struct GameProfile {
    pub username: String,
    pub uuid: Uuid,
    pub properties: Properties,
}

impl GameProfile {
    pub fn new(username: &str, uuid: Uuid) -> Self {
        Self {
            username: username.to_owned(),
            uuid: uuid,
            properties: Properties::new(),
        }
    }
}

impl Encode for GameProfile {
    fn encode(&self, writer: &mut Writer) {
        writer.write_uuid(&self.uuid);
        writer.write_str(&self.username);
        self.properties.encode(writer);
    }
}
