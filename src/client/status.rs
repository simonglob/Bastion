use crate::{network::state::ConnectionState, packets::{codec::write_packet, state::status::{StatusRequest, StatusResponse}}};

pub fn status(packet: StatusRequest, state: &mut ConnectionState) -> std::io::Result<Vec<u8>> {
    println!("ping packet...");
    
    Ok(write_packet(&StatusResponse {
        json: r#"{
            "version": {
                "name": "26.1",
                "protocol": 775
            },
            "players": {
                "max": 20,
                "online": 1,
                "sample": [
                    {
                        "name": "deez nuts",
                        "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                    }
                ]
            },
            "description": {
                "text": "hello"
            },
            "enforcesSecureChat": false
        }"#.to_owned()
    }))
}
