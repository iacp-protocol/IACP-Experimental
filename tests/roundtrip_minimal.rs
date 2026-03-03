use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::{thread, time::Duration};

fn pick_free_port() -> u16 {
    // Bind on port 0 lets the OS pick a free port.
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind 127.0.0.1:0 failed");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

fn start_agent_b(port: u16) -> std::process::Child {
    Command::new("cargo")
        .args(["run", "--bin", "agent_b"])
        .env("IACP_EXP_PORT", port.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start server")
}

fn connect_and_send(port: u16, payload: &[u8]) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream.write_all(payload).unwrap();

    let mut buffer = [0u8; 1024];
    let size = stream.read(&mut buffer).unwrap();

    String::from_utf8_lossy(&buffer[..size]).to_string()
}

#[test]
fn roundtrip_minimal() {
    let port = pick_free_port();
    let mut server = start_agent_b(port);

    thread::sleep(Duration::from_millis(500));

    let response = connect_and_send(port, b"structure_version=0.1");
    assert_eq!(response.trim(), "OK");

    let _ = server.kill();
    let _ = server.wait();
}

#[test]
fn roundtrip_duplicate_structure_version_observed_behavior() {
    let port = pick_free_port();
    let mut server = start_agent_b(port);

    thread::sleep(Duration::from_millis(500));

    // ===== Run #1 =====
    let first = {
        let response = connect_and_send(port, b"structure_version=0.1\nstructure_version=0.2");
        println!("dup structure_version response (run1) = {:?}", response);

        let trimmed = response.trim().to_string();
        assert!(trimmed == "OK" || trimmed == "ERR");
        trimmed
    };

    // ===== Run #2 (same input) =====
    let second = {
        let response = connect_and_send(port, b"structure_version=0.1\nstructure_version=0.2");
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