use bytes::{Buf, Bytes};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use Bastion::{
    network::session::Session,
    packets::{ClientPacket, codec::read_frame},
};

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
    let mut state = Session::new();
    let mut _buffer = [0u8; 1024];

    loop {
        let pos = socket.read(&mut _buffer).await?;
        if pos == 0 {
            return Ok(());
        }

        let mut buffer = Bytes::copy_from_slice(&_buffer[..pos]);
        let mut output: Vec<u8> = vec![];

        loop {
            let (length, id) = match read_frame(&mut buffer) {
                Some(frame) => frame,
                None => break, // nothing left to read
            };

            let packet = ClientPacket::from(id, state.get_state(), &mut buffer);
            println!("{:?}", packet);
            match ClientPacket::match_handler(packet, &mut state).await {
                Ok(data) => output.extend_from_slice(&data),
                Err(e) => {
                    eprintln!(
                        "packet {id:#04x} (state: {:?}) had an error: {:?}",
                        state.get_state(),
                        e
                    );
                    buffer.advance(length);
                }
            }
        }

        if !output.is_empty() {
            socket.write_all(&output).await?;
            socket.flush().await?;
        }
    }
}
