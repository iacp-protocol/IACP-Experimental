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
}
#[test]
fn roundtrip_duplicate_structure_version_observed_behavior() {
    // Start agent_b in background
    let mut server = Command::new("cargo")
        .args(["run", "--bin", "agent_b"])
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start server");

    thread::sleep(Duration::from_millis(500));

    // ===== Run #1 =====
    let first = {
        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

        // Duplicate structure_version (harness-only encoding, non-normative)
        stream
            .write_all(b"structure_version=0.1\nstructure_version=0.2")
            .unwrap();

        let mut buffer = [0u8; 1024];
        let size = stream.read(&mut buffer).unwrap();

        let response = String::from_utf8_lossy(&buffer[..size]).to_string();
        println!("dup structure_version response (run1) = {:?}", response);

        let trimmed = response.trim().to_string();
        assert!(trimmed == "OK" || trimmed == "ERR");
        trimmed
    };

    // ===== Run #2 (same input) =====
    let second = {
        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

        stream
            .write_all(b"structure_version=0.1\nstructure_version=0.2")
            .unwrap();

        let mut buffer = [0u8; 1024];
        let size = stream.read(&mut buffer).unwrap();

        let response = String::from_utf8_lossy(&buffer[..size]).to_string();
        println!("dup structure_version response (run2) = {:?}", response);

        let trimmed = response.trim().to_string();
        assert!(trimmed == "OK" || trimmed == "ERR");
        trimmed
    };

    // Determinism (same input -> same output)
    assert_eq!(first, second);

    let _ = server.kill();
    let _ = server.wait();
}