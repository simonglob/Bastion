use crate::{entities::player::GameProfile, packets::intent::Intent};
use reqwest::Client;
use rsa::{RsaPrivateKey, rand_core::OsRng};
use uuid::Uuid;

#[derive(Debug)]
pub struct Session {
    pub current_intent: Intent,
    pub private_key: RsaPrivateKey,
    pub verify_token: Option<Vec<u8>>,

    pub client: Client,

    pub profile: GameProfile,
}

impl Session {
    pub fn new() -> Self {
        Self {
            current_intent: Intent::Handshake,
            private_key: RsaPrivateKey::new(&mut OsRng, 1024).expect("failed to generate key"),
            verify_token: None,
            client: Client::new(),
            profile: GameProfile::new("", Uuid::nil()),
        }
    }

    pub fn update_state(&mut self, intent: Intent) {
        self.current_intent = intent;
    }

    pub fn get_state(&self) -> Intent {
        self.current_intent.clone()
    }
}
