use std::thread;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Write, Read, Error, BufReader, BufWriter};

fn handle_client(stream: TcpStream) -> Result<(), Error>{
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);
    // acknowledge the client has sent some data
    writer.write(b"0xA")?;
    writer.flush()?;
    let mut buff = Vec::new();
    loop {
        match reader.read_to_end(&mut buff) {
            Err(e) => println!("error {}", e),
            Ok(okdata) => {
                if okdata == 0 {
                    continue;
                }
            println!("{:?}", buff);
            }
        }
    }
}

fn main() {
    let bindaddr = SocketAddr::from(([0, 0, 0, 0], 6666));
    let listener = TcpListener::bind(&bindaddr).unwrap();
    println!("Server listening on port {}", bindaddr.port());
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
