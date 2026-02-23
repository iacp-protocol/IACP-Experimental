use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    // Harness-only encoding (non-normative)
    stream.write_all(b"structure_version=0.1").unwrap();

    let mut buffer = [0u8; 1024];
    let size = stream.read(&mut buffer).unwrap();

    let response = String::from_utf8_lossy(&buffer[..size]);
    println!("Response: {}", response);
}