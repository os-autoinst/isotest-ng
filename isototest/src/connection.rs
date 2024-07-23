// SPDX-FileCopyrightTest: Christopher Hock <christopher.hock@suse.com>
// SPDX-License-Identifier: GPL-2.0-or-later

//! # Connection Module
//!
//! This module handles the VncClient and its connection to the VncServer.
use tokio::{self, net::TcpStream};
use vnc::{PixelFormat, VncClient, VncConnector, VncError};

/// Create a new VNC client.
///
/// During the connection process the connection to the VNC server is
/// tested.
///
/// # Parameters
///
/// * target_ip: `String` - The IP and port of the VNC target server. (e.g `172.0.0.1:5900`)
/// * psw: `String` - The password used for authenticating with the server. (If the server
/// does not use authentication, this is irrelevant.)
///
/// # Returns
///
/// * vnc: `Ok(VncClient)` - A new instance of a `vnc-rs` `VncClient`.
/// * `Err(VncError)` - A `VncError` type, depending on the cause of failure.
///
/// # Panics
///
/// This function may panic, if the connection to the VNC server or the
/// configuration of the client fails.
pub async fn create_vnc_client(
    target_ip: String,
    mut psw: Option<String>,
) -> Result<VncClient, VncError> {
    if psw.is_none() {
        psw = Some(String::new());
    }

    let tcp: TcpStream = match TcpStream::connect(target_ip).await {
        Ok(tcp) => tcp,
        Err(e) => {
            let err = VncError::IoError(e);
            return Err(err);
        }
    };

    let vnc: VncClient = VncConnector::new(tcp)
        .set_auth_method(async move { Ok(psw.unwrap()) })
        .add_encoding(vnc::VncEncoding::Tight)
        .add_encoding(vnc::VncEncoding::Zrle)
        .add_encoding(vnc::VncEncoding::CopyRect)
        .add_encoding(vnc::VncEncoding::Raw)
        .allow_shared(true)
        .set_pixel_format(PixelFormat::bgra())
        .build()?
        .try_start()
        .await?
        .finish()?;

    Ok(vnc)
}

/// Stop VNC engine, release all resources
///
/// # Parameters
///
/// * client: `VncClient` - The client to kill.
///
/// # Returns
///
/// * `Ok(())` - In case the client terminates correctly.
/// * `Err(VncError)` - Escalates the `VncError` upwards, if the `.close()` function of `vnc-rs`
/// returns an error.
pub async fn kill_client(client: VncClient) -> Result<(), VncError> {
    match client.close().await {
        Ok(_) => {}
        Err(e) => return Err(e),
    };
    drop(client);
    Ok(())
}
