use crate::packets::intent::Intent;

#[derive(Debug)]
pub struct ConnectionState {
    current_intent: Intent,
}

impl ConnectionState {
    pub fn new() -> Self {
        Self {
            current_intent: Intent::Handshake,
        }
    }

    pub fn update_state(&mut self, intent: Intent) {
        self.current_intent = intent;
    }

    pub fn get_state(&self) -> Intent {
        self.current_intent.clone()
    }
}
