use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    let server = SocketAddr::from(([192, 168, 0, 14], 6666));
    match TcpStream::connect(&server) {
        Ok(mut stream) => {
            println!("Successfully connected to {0} on port {1}", server.ip(), server.port());
            let msg = b"hello";

            stream.write(msg).unwrap();

            println!("Sent {}, awaiting reply...", from_utf8(msg).unwrap());
            let mut data = [0 as u8; 5]; // 5 byte buffer

            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("terminated");
}
