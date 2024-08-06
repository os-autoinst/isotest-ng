// SPDX-FileCopyrightTest: Christopher Hock <christopher.hock@suse.com>
// SPDX-License-Identifier: GPL-2.0-or-later

//! This module handles the VncClient and its connection to the VncServer.
use log::{debug, error, info};
use tokio::{self, net::TcpStream};
use vnc::{PixelFormat, VncClient, VncConnector, VncError};

use crate::logging::LOG_TARGET;

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
    info!(target: LOG_TARGET, "Creating VNC client for target IP: '{}'", target_ip);

    if psw.is_none() {
        debug!("No password provided; using empty password.");
        psw = Some(String::new());
    }

    let tcp: TcpStream = match TcpStream::connect(target_ip).await {
        Ok(tcp) => tcp,
        Err(e) => {
            error!(target: LOG_TARGET, "Failed to connect: {}", e);
            let err = VncError::IoError(e);
            return Err(err);
        }
    };

    let vnc: VncClient = match VncConnector::new(tcp)
        .set_auth_method(async move { Ok(psw.unwrap()) })
        .add_encoding(vnc::VncEncoding::Tight)
        .add_encoding(vnc::VncEncoding::Zrle)
        .add_encoding(vnc::VncEncoding::CopyRect)
        .add_encoding(vnc::VncEncoding::Raw)
        .add_encoding(vnc::VncEncoding::Trle)
        .add_encoding(vnc::VncEncoding::CursorPseudo)
        .add_encoding(vnc::VncEncoding::DesktopSizePseudo)
        .allow_shared(true)
        // NOTE: If the color encoding is changed in the following line, you must also change it in
        // view.rs to avoid the saved screenshots from having swapped colors.
        .set_pixel_format(PixelFormat::rgba())
        .build()
    {
        Ok(vnc) => vnc,
        Err(e) => {
            error!(target: LOG_TARGET, "Failed to build VNC client: {}", e);
            return Err(e);
        }
    }
    .try_start()
    .await?
    .finish()?;

    info!("VNC Client successfully built and started.");

    Ok(vnc)
}

/// Stop VNC engine, release all resources.
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
    info!(target: LOG_TARGET, "Closing connection...");
    match client.close().await {
        Ok(_) => {
            info!(target: LOG_TARGET, "Connection closed.");
        }
        Err(e) => {
            error!(target: LOG_TARGET, "Unable to close connection: {}", e);
            return Err(e);
        }
    };
    drop(client);
    info!(target: LOG_TARGET, "Client dropped.");
    Ok(())
}
