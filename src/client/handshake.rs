use crate::packets::{client::{Handshake as ClientHandshake, Ping}, codec::write_packet, server::Handshake};

pub fn handshake(packet: ClientHandshake) -> std::io::Result<Vec<u8>> {
    println!("{:?}", packet);

    // taken from protocol docs
    Ok(write_packet(&Handshake {
json: r#"{"version":{"name":"1.21.6","protocol":775},"players":{"max":20,"online":0,"sample":[]},"description":"","enforcesSecureChat":false}"#.to_owned()
    }))
}

pub fn pong(packet: Ping) -> std::io::Result<Vec<u8>> {
    println!("{:?}", packet);
    Ok(vec![])
}