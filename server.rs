// imports
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};
use std::str;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut data = [0 as u8; 50]; // 50 byte buffer
    loop {
        let nbytes = stream.read(&mut data)?;
        if nbytes == 0 {
            return Ok(());
        }
        /*
        for i in 0..nbytes {
            print!("{}", str::from_utf8(data[i]).unwrap());
        }
        println!("");
        */
        println!("Client sent: {}", str::from_utf8(&data).unwrap());
        stream.write(&data[..nbytes])?;
        stream.flush()?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6666").unwrap();
    println!("Server listening on port 6666");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
