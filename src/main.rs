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
    let shared: SharedState = Arc::new(Mutex::new(HashMap::new()));
    let mut conn_id = 0u64;

    println!("Listening on 127.0.0.1:3000");

    loop {
        let (socket, _) = listener.accept().await?;
        let state = shared.clone();
        conn_id += 1;

        {
            let mut map = state.lock().await;
            map.insert(conn_id, ConnectionState::new());
        }

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, conn_id, state).await {
                eprintln!("Connection error: {e}")
            }
        });
    }
}

async fn handle_connection(
    mut socket: tokio::net::TcpStream,
    conn_id: u64,
    shared: SharedState,
) -> std::io::Result<()> {
    let mut buffer = [0u8; 1024];
    println!("Got connection!");

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

            {
                let mut map = shared.lock().await;
                if let Some(state) = map.get_mut(&conn_id) {
                    match DISPATCHER.dispatch(id, &mut reader, state) {
                        Ok(data) => output.extend_from_slice(&data),
                        Err(e) => {
                            reader.peek(length);
                            eprintln!("packet {id:#04x} had an error: {:?}", e);
                        }
                    }
                }
            }

            if !output.is_empty() {
                socket.write_all(&output).await?;
                socket.flush().await?;
            }
        }
    }
}
