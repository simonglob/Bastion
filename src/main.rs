mod packets;
mod client;

use std::sync::LazyLock;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

use crate::{client::handshake::{handshake, pong}, packets::{
    codec::read_frame, dispatcher::PacketDispatcher, reader::Reader,
}};

static DISPATCHER: LazyLock<PacketDispatcher> = LazyLock::new(|| {
    let mut dispatcher = PacketDispatcher::default();

    dispatcher.register(handshake);
    dispatcher.register(pong);

    dispatcher
});

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
    let mut buffer = [0u8; 1024];
    println!("Got connection!");

    loop {
        let pos = socket.read(&mut buffer).await?;
        let mut reader = Reader::new(&buffer[..pos]);
        let mut output = vec![];

        loop {
            let (length, id) = match read_frame(&mut reader) {
                Some(frame) => frame,
                None => break, // nothing left to read
            };

            // if reader.is_empty() {
            //     eprintln!("packet already empty after reading headers");
            //     break
            // }
            
            match DISPATCHER.dispatch(id, &mut reader) {
                Ok(data) => output.extend_from_slice(&data),
                Err(e) => {
                    reader.peek(length);
                    eprintln!("packet {id:#04x} had an error: {:?}", e);
                }
            }

            println!("found a packet with packet id {id} and length {length}. the position after presumably parsing the rest of the packet is {:?}", reader.get_pos());
        }
        socket.write_all(&output).await?;
        socket.flush().await?;
    }
}
