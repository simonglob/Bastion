#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Intent {
    Handshake,
    Status,
    Login,
}

impl Intent {
    pub fn from_value(value: i32) -> Self {
        match value {
            0 => Self::Handshake,
            1 => Self::Status,
            2 => Self::Login,
            _ => Self::Handshake,
        }
    }
}
