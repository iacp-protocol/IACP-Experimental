use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use iacp_core::{validate, Field, MessageInput, OpaqueValue};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let size = stream.read(&mut buffer).unwrap();

    let received = String::from_utf8_lossy(&buffer[..size]).to_string();

    // Harness-only encoding (non-normative)
    // Expected format: structure_version=0.1
    let value = received.trim().strip_prefix("structure_version=").unwrap_or("");

    let msg = MessageInput {
        fields: vec![Field {
            name: "structure_version".to_string(),
            value: OpaqueValue::String(value.to_string()),
        }],
    };

    let _ = validate(msg);

    stream.write_all(b"OK").unwrap();
}

fn main() {
    // Harness-only config (non-normative)
    let port = env::var("IACP_EXP_PORT").unwrap_or_else(|_| "7878".to_string());
    let addr = format!("127.0.0.1:{port}");

    let listener = TcpListener::bind(&addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| handle_client(stream));
    }
}