use std::process::{Command, Stdio};
use std::{thread, time::Duration};
use std::net::TcpStream;
use std::io::{Read, Write};

#[test]
fn roundtrip_minimal() {
    // Start agent_b in background
    let mut server = Command::new("cargo")
        .args(["run", "--bin", "agent_b"])
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start server");

    thread::sleep(Duration::from_millis(500));

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    stream.write_all(b"structure_version=0.1").unwrap();

    let mut buffer = [0u8; 1024];
    let size = stream.read(&mut buffer).unwrap();

    let response = String::from_utf8_lossy(&buffer[..size]);
    assert_eq!(response, "OK");

    let _ = server.kill();
let _ = server.wait();
}

#[test]
fn roundtrip_invalid_missing_structure_version_yields_err() {
    let mut server = Command::new("cargo")
        .args(["run", "--bin", "agent_b"])
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start server");

    thread::sleep(Duration::from_millis(500));

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    // No structure_version prefix
    stream.write_all(b"x=1").unwrap();

    let mut buffer = [0u8; 1024];
    let size = stream.read(&mut buffer).unwrap();

    let response = String::from_utf8_lossy(&buffer[..size]);
    assert_eq!(response, "ERR");

    let _ = server.kill();
let _ = server.wait();
}