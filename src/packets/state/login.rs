use bytes::Bytes;
use uuid::Uuid;

use crate::{
    entities::player::GameProfile,
    packets::{
        codec::{Decode, Encode, PacketID},
        reader::MinecraftProtocol,
        writer::Writer,
    },
};

#[derive(Debug)]
pub struct LoginStart {
    pub username: String,
    pub uuid: Uuid,
}

impl PacketID for LoginStart {
    fn id() -> u16 {
        0x00
    }
}

impl Decode for LoginStart {
    fn decode(reader: &mut Bytes) -> std::io::Result<Self> {
        Ok(Self {
            username: reader.get_string(),
            uuid: reader.get_uuid(),
        })
    }
}

pub struct Disconnect {
    pub reason: String,
}

impl PacketID for Disconnect {
    fn id() -> u16 {
        0x00
    }
}

impl Encode for Disconnect {
    fn encode(&self, writer: &mut Writer) {
        writer.write_str(&self.reason);
    }
}

pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_key: Vec<u8>,
    pub should_authenticate: bool,
}

impl PacketID for EncryptionRequest {
    fn id() -> u16 {
        0x01
    }
}

impl Encode for EncryptionRequest {
    fn encode(&self, writer: &mut Writer) {
        writer.write_str(&self.server_id);
        writer.write_byte_array(&self.public_key);
        writer.write_byte_array(&self.verify_key);
        writer.write_u8(self.should_authenticate as u8);
    }
}

#[derive(Debug)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl PacketID for EncryptionResponse {
    fn id() -> u16 {
        0x01
    }
}

impl Decode for EncryptionResponse {
    fn decode(reader: &mut Bytes) -> std::io::Result<Self> {
        Ok(Self {
            shared_secret: reader.get_bytearray(),
            verify_token: reader.get_bytearray(),
        })
    }
}

pub struct LoginSuccess {
    pub profile: GameProfile,
}

impl PacketID for LoginSuccess {
    fn id() -> u16 {
        0x02
    }
}

impl Encode for LoginSuccess {
    fn encode(&self, writer: &mut Writer) {
        self.profile.encode(writer);
    }
}
