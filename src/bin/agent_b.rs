use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

use iacp_core::{Field, MessageInput, OpaqueValue, validate};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let size = stream.read(&mut buffer).unwrap();

    let received = String::from_utf8_lossy(&buffer[..size]).to_string();

    // Harness-only encoding (non-normative)
    // Expected format: structure_version=0.1
    let mut fields: Vec<Field> = Vec::new();

    if let Some(value) = received.trim().strip_prefix("structure_version=") {
        fields.push(Field {
            name: "structure_version".to_string(),
            value: OpaqueValue::String(value.to_string()),
        });
    }

    let msg = MessageInput { fields };

    let r = validate(msg);

    if r.is_ok() {
        stream.write_all(b"OK").unwrap();
    } else {
        stream.write_all(b"ERR").unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| handle_client(stream));
    }
}