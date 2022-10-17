use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use std::env::args;

fn read_file(filename: &str) -> Vec<u8> {
    let path = Path::new(&filename);
    let mut data = Vec::new();
    let mut file = File::open(path).expect("file not found");
    match file.read_to_end(&mut data) {
        Err(e) => eprintln!("{:?}", e),
        _ => ()
    }
    return data;
}

fn main() {
    let arguments: Vec<String> = args().collect();
    let server = SocketAddr::from(([192, 168, 0, 14], 6666));
    let filename = &arguments[1];
    println!("Reading and sending file: {}", filename);
    match TcpStream::connect(&server) {
        Ok(mut stream) => {
            println!("Successfully connected to {0} on port {1}", server.ip(), server.port());

            let msg = read_file(&filename);
            match stream.write(&msg) {
                Err(e) => eprintln!("{}", e),
                _ => ()
            }
            match stream.flush() {
                Err(e) => eprintln!("{}", e),
                _ => ()
            }
            println!("Sent {}, awaiting reply...", filename);
            let magic = b"0xA";
            let mut data = vec![0 as u8; magic.len()];
            match stream.read(&mut data) {
                Ok(_) => {
                    if &data == &magic {
                        println!("Server successfully received {}", filename);
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
