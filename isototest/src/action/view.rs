//! # View module
//!
//! This module handles everything related to requesting visual data from the VNC server.
use image::{DynamicImage, GenericImage, ImageFormat, Rgba};
use std::{
    path::Path, sync::{Arc, Mutex}, time::{Duration, Instant}
};

use image::ImageBuffer;

use image::DynamicImage::ImageRgba8;
use vnc::{Rect, VncClient, VncError, VncEvent, X11Event};

use log::{error, info, warn};

use crate::logging::LOG_TARGET;

/// Receive a screenshot of the remote machine.
///
/// # Parameters
///
/// * client: `&VncClient` - The client instance used for connection.
/// * file_path: `&str` - A file path you want to save your screenshot under as a `str`.
/// * resolution: `Option<u32, u32>` - The resolution of the VNC session.
/// * timeout: `Duration` - The `Duration` this function should wait for a `VncEvent` before it
/// continues.
/// * compose: `bool` - Toggle wheather to overlay new image data onto the previous image to create
/// a full, updated new screenshot.
///
/// **NOTE**: The `resolution` must be passed to all calls of `read_screen` except the first one.
/// If it is not passed, the function will attempt to detect the resolution from the VNC server.
/// This only works for the first time though. The client cannot retrieve the resolution a second
/// time by itself as long as it has not changed. We recommend to save the `Ok()` return value of
/// the function so you have a global resolution state to return to when calling.
///
/// # Returns
///
/// * `Ok((u32, u32))` - The resolution of the VNC machine we connect to.
/// * `Err(VncError)` - Variation of `VncError` if something goes wrong.
pub async fn read_screen(
    client: &VncClient,
    file_path: &str,
    resolution: Option<(u32, u32)>,
    timeout: Duration,
    compose: bool,
) -> Result<(u32, u32), VncError> {
    info!(target: LOG_TARGET, "Requesting screenshot...");
    // Request screen update.
    client.input(X11Event::Refresh).await?;

    let mut img_parts: Vec<(Rect, Vec<u8>)> = Vec::new();
    let mut width: Option<u32>;
    let mut height: Option<u32>;

    // Try to detect screen resolution of the remote machine if it has not been passed.
    // **This will cause issues, if you try to use this functionality a second time.**
    match resolution {
        Some((x, y)) => {
            info!(target: LOG_TARGET, "Resolution provided; proceeding...");
            width = Some(x);
            height = Some(y);
        }
        None => match client.recv_event().await? {
            VncEvent::SetResolution(screen) => {
                info!(target: LOG_TARGET, "Resolution received. Screen resolution: {}x{}", screen.width, screen.height);
                width = Some(screen.width as u32);
                height = Some(screen.height as u32);

                client.input(X11Event::Refresh).await?;
            }
            _ => {
                error!(target: LOG_TARGET, "Failed to retrieve screen resolution. Aborting...");
                return Err(VncError::General(
                    "[error] No resolution found!".to_string(),
                ));
            }
        },
    }

    let path: &Path = Path::new(file_path);
    let idle_timer: Instant = Instant::now();
    let prev_image: Option<Arc<Mutex<DynamicImage>>> = if compose {
        let img = DynamicImage::new_rgba8(width.unwrap(), height.unwrap());
        Some(Arc::new(Mutex::new(img)))
    } else {
        None
    };

    loop {
        // Poll new vnc events.
        match client.poll_event().await? {
            Some(x) => match x {
                VncEvent::SetResolution(screen) => {
                    info!(target: LOG_TARGET, "Screen resolution: {}x{}", screen.width, screen.height);
                    width = Some(screen.width as u32);
                    height = Some(screen.height as u32);

                    client.input(X11Event::Refresh).await?;
                }
                VncEvent::RawImage(rect, data) => {
                    img_parts.push((rect, data));
                }
                VncEvent::Error(e) => {
                    error!(target: LOG_TARGET, "Error event received: {}", e);
                    return Err(VncError::General(e));
                }
                x => {
                    warn!(target: LOG_TARGET,
                        "Function 'read_screen' got unexpected event '{:?}'.",
                        x
                    );
                    break;
                }
            },
            None => {
                if idle_timer.elapsed() >= timeout {
                    warn!(target: LOG_TARGET, "Timeout while waiting for VNC Event.");
                    break;
                }
            }
        }
    }

    let mut image: ImageBuffer<Rgba<u8>, _> = ImageBuffer::new(width.unwrap(), height.unwrap());

    // Reconstruct image from snippets sent by VNC server.
    for (rect, data) in img_parts {
        let mut view = image.sub_image(
            rect.x as u32,
            rect.y as u32,
            rect.width as u32,
            rect.height as u32,
        );
        let image_buffer: ImageBuffer<Rgba<u8>, _> =
            ImageBuffer::from_raw(rect.width as u32, rect.height as u32, data.to_vec())
                .ok_or("Failed to create image buffer!")
                .unwrap();

        for x in 0..rect.width {
            for y in 0..rect.height {
                view.put_pixel(
                    x as u32,
                    y as u32,
                    image_buffer.get_pixel(x as u32, y as u32).to_owned(),
                );
            }
        }
    }

    // TODO: If compose flag set, compose the new image onto the old one.

    // Save image to file system in PNG format.
    // NOTE: If the image color encoding is changed here, you must also change it in connection.rs!
    ImageRgba8(image)
        .save_with_format(path, ImageFormat::Png)
        .unwrap();

    info!(target: LOG_TARGET, "Screenshot saved to '{}'", file_path);
    Ok((width.unwrap(), height.unwrap()))
}

/// Compose new image data to the previous image and save copy.
///
/// This is needed for subsequent calls of `read_screen` as the VNC server will only return the
/// last pixels changed since the previous request.
/// While this is handy for performance, it is no recommended for our use case as we need to have a
/// full picture of the screen to compare the current state of the test worker against expected
/// output.
///
/// # Parameters
///
/// * prev_image: `Arc<Mutex<DynamicImage>>` - The previous image the new data must be layed on top
/// of.
/// * rect: `Rect` - The rectangle specifying the area and position of the new data.
/// * data: `Vec<u8>` - The mew üoxeö data that needs to be layed ontop of the old image.
///
/// # Returns
///
/// * `Ok(())` - If the operation was successful.
/// * `Err(VncError)` - If the parsing or combination of the images fails.
fn compose_image(
    prev_image: Arc<Mutex<DynamicImage>>,
    rect: Rect,
    data: Vec<u8>,
) -> Result<(), VncError> {
    info!(target: LOG_TARGET, "Combining new pixel data with previous image.");

    // Create a new buffer representing the new image data to be comibined onto the old image
    let image_buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
        rect.width as u32,
        rect.height as u32,
        data,
    ).ok_or_else(|| VncError::General("Failed to create image buffer.".to_string()))?;

    // Lock the previous image to gain mutable access on a new thread.
    let mut prev_image = prev_image.lock().map_err(|_| VncError::General("Failed to lock prev_image.".to_string()))?;

    // Iterate over the new pixel data and update the corresponding iamge data
    for x in 0..rect.width {
        for y in 0..rect.height {
            // Update the pixel in the previous image at the specified position
            prev_image.put_pixel(
                (rect.x + x) as u32,
                (rect.y + y) as u32,
                image_buffer.get_pixel(x as u32, y as u32).to_owned(),
            );
        }
    }
    Ok(())
}
