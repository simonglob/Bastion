use bytes::Bytes;

use crate::{
    events::{handshake, login, status},
    network::session::Session,
    packets::{
        codec::Decode,
        intent::Intent,
        state::{
            handshake::Handshake,
            login::{EncryptionResponse, LoginStart},
            status::{PingPong, StatusRequest},
        },
    },
};

pub mod state;

pub mod reader;
pub mod writer;

pub mod codec;
pub mod intent;

pub type VarInt = i32;
#[derive(Debug)]
pub enum ClientPacket {
    Handshake(Handshake),
    StatusRequest(StatusRequest),
    Ping(PingPong),
    LoginStart(LoginStart),
    EncryptionResponse(EncryptionResponse),
}

macro_rules! decode_client_packet {
    ($id:expr, $intent:expr, $buffer:expr; $(($pid:pat, $pintent:path) => $variant:ident($ty:ty)),+ $(,)?) => {
    match ($id, $intent) {
        $(
        ($pid, $pintent) => Self::$variant(<$ty as Decode>::decode($buffer).unwrap()),
        )+
        _ => panic!("unknown packet id/intent combination"),
    }
    };
}

macro_rules! handle_client_packet {
    ($packet:ident, $state:expr; $($variant:ident => $handler:path),+ $(,)?) => {
        match $packet {
            $(
            Self::$variant(p) => $handler(p, $state).await,
            )+
        }
    };
}

impl ClientPacket {
    pub fn from(id: u16, intent: Intent, buffer: &mut Bytes) -> Self {
        decode_client_packet!(id, intent, buffer;
            (0x00, Intent::Handshake) => Handshake(Handshake),

            (0x00, Intent::Status) => StatusRequest(StatusRequest),
            (0x01, Intent::Status) => Ping(PingPong),

            (0x00, Intent::Login) => LoginStart(LoginStart),
            (0x01, Intent::Login) => EncryptionResponse(EncryptionResponse),
        )
    }

    pub async fn match_handler(
        packet: ClientPacket,
        state: &mut Session,
    ) -> std::io::Result<Vec<u8>> {
        handle_client_packet!(packet, state;
            Handshake => handshake::handshake,

            StatusRequest => status::status,
            Ping => status::ping,

            LoginStart => login::login,
            EncryptionResponse => login::encryption
        )
    }
}
