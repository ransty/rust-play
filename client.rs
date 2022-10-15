use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("192.168.0.14:6666") {
        Ok(mut stream) => {
            println!("Successfully connected to 192.168.0.14 on port 6666");
            let msg = b"hello";

            stream.write(msg).unwrap();

            println!("Sent hello, awaiting reply...");

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
