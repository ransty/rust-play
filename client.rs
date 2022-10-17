use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::fs::File;
use std::fs;
use std::path::Path;
use std::env::args;
use std::process;

fn read_file(filename: &str) -> Vec<u8> {
    let path = Path::new(&filename);
    let mut data = Vec::new();
    let mut file = File::open(path).expect("file went MIA");
    match file.read_to_end(&mut data) {
        Err(e) => eprintln!("{:?}", e),
        _ => ()
    }
    data
}

struct FileTransferConfig {
    filepath: String
}

impl FileTransferConfig {
    fn build(arguments: &[String]) -> Result<FileTransferConfig, &'static str> {
        if arguments.len() < 2 {
            return Err("You need to specify a file!");
        }
        let filepath = arguments[1].clone();
        Ok(FileTransferConfig { filepath })
    }
}


fn handle_ftc(ftc: &FileTransferConfig) -> bool {
    let fsmetadata = fs::metadata(ftc.filepath.clone()).unwrap_or_else(|err| {
        println!("Problem reading metadata for file \"{0}\": {err}", ftc.filepath);
        process::exit(1);
    });
    
    if fsmetadata.is_file() == false {
        println!("Problem reading file: \"{}\" is not a file", ftc.filepath);
        process::exit(1);
    }
    true
}

fn main() {
    let arguments: Vec<String> = args().collect();

    let ftc = FileTransferConfig::build(&arguments).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    assert!(handle_ftc(&ftc));

    let filename = ftc.filepath;

    let server = SocketAddr::from(([192, 168, 0, 14], 6666));
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
