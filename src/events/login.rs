use num_bigint::BigInt;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey, pkcs8::EncodePublicKey};
use sha1::{Digest, Sha1};

use crate::{
    entities::player::GameProfile,
    network::session::Session,
    packets::{
        codec::write_packet,
        state::login::{
            Disconnect, EncryptionRequest, EncryptionResponse, LoginStart, LoginSuccess,
        },
    },
};

pub async fn login(packet: LoginStart, state: &mut Session) -> std::io::Result<Vec<u8>> {
    let public_key = RsaPublicKey::from(&state.private_key);
    let verify_key = rand::random::<[u8; 4]>().to_vec();
    state.verify_token = Some(verify_key.clone());
    state.profile.username = packet.username;
    state.profile.uuid = packet.uuid;

    let packet = write_packet(&EncryptionRequest {
        server_id: String::new(),
        public_key: public_key.to_public_key_der().unwrap().as_ref().to_vec(),
        verify_key: verify_key,
        should_authenticate: true,
    });

    Ok(packet)
}

fn notchian_hash(username: &str) -> String {
    BigInt::from_signed_bytes_be(&Sha1::digest(username)).to_str_radix(16)
}

pub async fn encryption(
    packet: EncryptionResponse,
    state: &mut Session,
) -> std::io::Result<Vec<u8>> {
    let notchian = notchian_hash(&state.profile.username);
    let response = state
        .client
        .post("https://sessionserver.mojang.com/session/minecraft/join")
        .body(notchian)
        .send()
        .await;
    println!("{:?}", response);

    let decrypted_verify_token = state
        .private_key
        .decrypt(Pkcs1v15Encrypt, &packet.verify_token)
        .unwrap();

    if decrypted_verify_token != state.verify_token.clone().unwrap() {
        println!("Should disconnect....");
        return Ok(write_packet(&Disconnect {
            reason: r#"{"text": "Hello World"}"#.to_owned(),
        }));
    }

    let profile = &state.profile;
    let _packet = write_packet(&LoginSuccess {
        profile: GameProfile::new(&profile.username, profile.uuid),
    });
    // state changes to configuration
    Ok(_packet)
}
