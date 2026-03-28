mod io;

use tokio::{
    io::{AsyncReadExt},
    net::{TcpListener},
};

use crate::io::reader::Reader;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on 127.0.0.1:3000");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 1024];
            
            let pos = socket
                .read(&mut buffer)
                .await
                .expect("failed to read data");

            let body = &buffer[..pos];
            let mut packet = Reader::new(body);

            let length = packet.read_varint();
            let packet_id = packet.read_varint(); 

            if packet_id != 0 {
                return;
            }

            let protocol_version = packet.read_varint();
            let server_addr = packet.read_string();
            let port = packet.read_u16();
            let intent = packet.read_varint();
            println!(
                "length: {:?} packet_id: {:?} protocol_version: {:?} server_addr: {:?} port: {:?} intent: {:?}",
                length, packet_id, protocol_version, server_addr, port, intent
            );
        });
    }
}
