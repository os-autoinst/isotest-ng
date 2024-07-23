//! This module contains common setup code required for the testuite.
use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use vnc::VncError;

/// Mock VNC server.
pub async fn start_mock_vnc_srv() -> Result<TcpListener, VncError> {
    eprintln!("Starting mock VNC server...");
    let srv = match TcpListener::bind("127.0.0.1:0").await {
        Ok(srv) => {
            eprintln!("Mock VNC server started on {:?}", srv.local_addr());
            Ok(srv)
        }
        Err(e) => return Err(VncError::IoError(e)),
    };
    srv
}

// TODO: Implement mock vnc server handshake.
pub async fn mock_vnc_handshake(mut socket: TcpStream) -> Result<(), io::Error> {
    eprintln!("Starting VNC handshake...");

    // Send the protocol version response
    let response = b"RFB 003.003\n";
    eprintln!("VNC Version response created '{:?}'.", response);
    if let Err(e) = socket.write_all(response).await {
        eprintln!("Failed to write protocol version response: {}", e);
        return Err(e);
    }

    eprintln!("Sent protocol version response.");
    // Read the client's protocol version message (12 bytes)
    let mut buff = [0; 12];
    eprintln!("First message buffer created '{:?}'.", &buff);
    match socket.read_exact(&mut buff).await {
        Ok(_) => eprintln!("Received protocol version: {:?}", &buff),
        Err(e) => {
            eprintln!("Failed to read protocol version: {}", e);
            return Err(e);
        }
    }

    // Verify the protocol version
    let version_message = String::from_utf8_lossy(&buff);
    if !version_message.starts_with("RFB 003.003") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unsupported VNC protocol version",
        ));
    }

    // Send the security type response (0 for successful authentication)
    let auth_response = b"\0\0\0\0\0\0\0\0";
    eprintln!("Created security response buffer '{:?}'.", &auth_response);
    if let Err(e) = socket.write_all(auth_response).await {
        eprintln!("Failed to write security type response: {}", e);
        return Err(e);
    }
    eprintln!("Sent security type response.");

    // Read the client's security type message (8 bytes)
    let mut buf = [0; 8];
    eprintln!("Created security receiving buffer '{:?}'.", &buf);
    match socket.read_exact(&mut buf).await {
        Ok(_) => eprintln!("Received security type: {:?}", &buf),
        Err(e) => {
            eprintln!("Failed to read security type: {}", e);
            return Err(e);
        }
    }

    // Check if the security type is supported (0 for no authentication)
    if buf[0] != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unsupported security type",
        ));
    }

    // Send the initialization response (example response)
    let init_response = [0; 4];
    if let Err(e) = socket.write_all(&init_response).await {
        eprintln!("Failed to write initialization response: {}", e);
        return Err(e);
    }
    eprintln!("Sent initialization response.");

    // Read the client's initialization message (16 bytes as an example)
    let mut buf = [0; 16];
    match socket.read_exact(&mut buf).await {
        Ok(_) => eprintln!("Received initialization message: {:?}", &buf),
        Err(e) => {
            eprintln!("Failed to read initialization message: {}", e);
            return Err(e);
        }
    }

    // Properly shutdown the connection
    if let Err(e) = socket.shutdown().await {
        eprintln!("Failed to shutdown the connection: {}", e);
        return Err(e);
    }
    eprintln!("Connection shutdown successfully.");

    Ok(())
}

/// Kill server connection.
pub async fn kill_connection(mut socket: tokio::net::TcpStream) {
    let _ = socket.shutdown();
}
