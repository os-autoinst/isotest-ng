use std::net::SocketAddr;
use isototest::connection::kill_client;
use tokio::net::TcpListener;
use vnc::client;
use vnc::PixelFormat;
use vnc::VncError;

use isototest::connection::create_vnc_client;
mod common;

#[tokio::test]
#[ignore = "Broken, needs to be fixed."]
async fn test_create_success() {
    let srv = common::start_mock_vnc_srv()
        .await
        .expect("Failed to start mock VNC server");
    let addr = srv.local_addr().unwrap().to_string();
    let psw = Some("password".to_string());

    // Spawn a task to handle the VNC handshake
    let handshake_task = tokio::spawn(async move {
        if let Ok((socket, _)) = srv.accept().await {
            common::mock_vnc_handshake(socket)
                .await
                .expect("Failed during mock VNC handshake");
        }
    });

    // Create the VNC client
    let result = create_vnc_client(addr, psw).await;
    match result {
        Ok(_) => assert!(true),
        Err(e) => panic!("{}", e),
    };

    // Await the handshake task to ensure it completes
    handshake_task
        .await
        .expect("Failed to await handshake task");
}

#[tokio::test]
async fn test_create_invalid_ip() {
    let target_ip = "256.256.256.256:5900".to_string();
    let psw = Some("pass".to_string());
    let result = create_vnc_client(target_ip, psw).await;
    assert!(matches!(result, Err(VncError::IoError(_))));
}

#[tokio::test]
#[ignore = "Broken, needs to be fixed."]
async fn test_create_no_pass() {
    let srv = common::start_mock_vnc_srv()
        .await
        .expect("Failed to start mock VNC server");
    let addr = srv.local_addr().unwrap().to_string();
    let psw = None;

    // Spawn a task to handle the VNC handshake
    let handshake_task = tokio::spawn(async move {
        if let Ok((socket, _)) = srv.accept().await {
            common::mock_vnc_handshake(socket)
                .await
                .expect("Failed during mock VNC handshake");
        }
    });

    // Create the VNC client
    let result = create_vnc_client(addr, psw).await;
    assert!(result.is_ok());

    // Await the handshake task to ensure it completes
    handshake_task
        .await
        .expect("Failed to await handshake task");
}

#[tokio::test]
async fn test_create_connect_fail() {
    let target_ip = "127.0.0.1:9999".to_string();
    let psw = Some("password".to_string());

    let result = create_vnc_client(target_ip, psw).await;
    assert!(matches!(result, Err(VncError::IoError(_))));
}

#[tokio::test]
#[ignore = "Broken, needs to be fixed."]
async fn test_client_kill() {
    let srv = common::start_mock_vnc_srv().await.expect("Failed to start VNC server");
    let addr = srv.local_addr().unwrap().to_string();

    let client = create_vnc_client(addr, None).await.expect("[Error] Test 'kill_client' failed. Unable to create client.");
    let result = kill_client(client).await;

    assert!(result.is_ok());
}
