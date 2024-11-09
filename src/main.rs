// use std::collections::HashMap;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    let mut buf = [0; 1024];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                let received = &buf[..size];
                println!("Received {} bytes from {}", size, src);
                println!("Data: {:?}", String::from_utf8_lossy(received));
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
            }
        }
    }
}
