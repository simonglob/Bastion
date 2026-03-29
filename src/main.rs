mod client;
mod network;
mod packets;

use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

use crate::{
    client::{handshake::handshake, ping::ping, status::status},
    network::state::ConnectionState,
    packets::{codec::read_frame, dispatcher::PacketDispatcher, intent::Intent, reader::Reader},
};

static DISPATCHER: LazyLock<PacketDispatcher> = LazyLock::new(|| {
    let mut dispatcher = PacketDispatcher::default();

    dispatcher.register(handshake, Intent::Handshake);
    dispatcher.register(status, Intent::Status);
    dispatcher.register(ping, Intent::Status);

    dispatcher
});

type SharedState = Arc<Mutex<HashMap<u64, ConnectionState>>>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Listening on 127.0.0.1:3000");

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Connection error: {e}")
            }
        });
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream) -> std::io::Result<()> {
    let mut state = ConnectionState::new();
    let mut buffer = [0u8; 1024];

    loop {
        let pos = socket.read(&mut buffer).await?;
        if pos == 0 {
            return Ok(());
        }

        let mut reader = Reader::new(&buffer[..pos]);
        let mut output = vec![];

        loop {
            let (length, id) = match read_frame(&mut reader) {
                Some(frame) => frame,
                None => break, // nothing left to read
            };

            match DISPATCHER.dispatch(id, &mut reader, &mut state) {
                Ok(data) => output.extend_from_slice(&data),
                Err(e) => {
                    reader.peek(length);
                    eprintln!(
                        "packet {id:#04x} (state: {:?}) had an error: {:?}",
                        state.get_state(),
                        e
                    );
                }
            }
        }

        if !output.is_empty() {
            socket.write_all(&output).await?;
            socket.flush().await?;
        }
    }
}
