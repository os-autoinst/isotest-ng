//! # View module
//!
//! This module handles everything related to requesting visual data from the VNC server.
use chrono::Utc;
use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat, Rgba};
use image::{ImageBuffer, RgbaImage};
use std::path::PathBuf;
use std::{
    env,
    path::Path,
    time::{Duration, Instant},
};
use vnc::{Rect, VncClient, VncError, VncEvent, X11Event};

use log::{error, info, warn};

use crate::logging::LOG_TARGET;

/// Receive a screenshot of the remote machine.
///
/// # Parameters
///
/// * client: `&VncClient` - The client instance used for connection.
/// * file_path: `Option<&Path>` - A file path you want to save your screenshot under as a `&Path`.
/// (If `None` -> `CWD` is set as output dir.)
/// * resolution: `Option<u32, u32>` - The resolution of the VNC session.
/// * timeout: `Duration` - The `Duration` this function should wait for a `VncEvent` before it
/// continues.
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
    file_path: Option<&Path>,
    resolution: Option<(u32, u32)>,
    timeout: Duration,
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

    let idle_timer: Instant = Instant::now();

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

    let mut prefix: PathBuf;
    match file_path {
        Some(x) => {
            prefix = x.to_owned();
        }
        None => {
            let dir = env::current_dir()?;
            prefix = dir.to_owned();
        }
    }

    let last_modified_file: Option<std::fs::DirEntry> = std::fs::read_dir(&prefix)
        .expect("Couldn't access local directory")
        .flatten() // Remove failed
        .filter(|f| f.metadata().unwrap().is_file()) // Filter out directories (only consider files)
        .max_by_key(|x| x.metadata().unwrap().modified().unwrap()); // Get the most recently modified file

    prefix.push(format!("frame_{}.png", Utc::now()));

    match last_modified_file {
        Some(x) => {
            let prev_image: DynamicImage = image::open(x.path()).unwrap();
            // Layer the image data on top of the previous image.
            let composed_image: DynamicImage =
                compose_image(&prev_image, &DynamicImage::ImageRgba8(image));
            // image = prev_image.to_rgba8()
            composed_image
                .save_with_format(&prefix, ImageFormat::Png)
                .unwrap();
        }
        None => {
            // Save image to file system in PNG format.
            // NOTE: If the image color encoding is changed here, you must also change it in connection.rs!
            DynamicImage::ImageRgba8(image)
                .save_with_format(&prefix, ImageFormat::Png)
                .unwrap();
        }
    }

    info!(target: LOG_TARGET, "Screenshot saved to '{}'", prefix.to_str().unwrap());
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
/// * `base: &DynamicImage` - The image you want to lay the delta on top of.
/// * `overlay: &DynamicImage` - The new image data, which only includes this pixels that have
/// changed since the last screen request.
///
/// # Returns
///
/// * `DynamicImage` - New `DynamicImage` consisting of the already read parts of the screen
/// overlayed with the requested delta.
fn compose_image(prev_image: &DynamicImage, new: &DynamicImage) -> DynamicImage {
    info!(target: LOG_TARGET, "Combining new pixel data with previous image.");
    let (width, height): (u32, u32) = (prev_image.width(), prev_image.height());
    let mut new_image: RgbaImage = prev_image.to_rgba8();

    for x in 0..width {
        for y in 0..height {
            let base_pixel: Rgba<u8> = prev_image.get_pixel(x, y);
            let overlay_pixel: Rgba<u8> = new.get_pixel(x, y);
            new_image.put_pixel(x, y, blend_pixels(base_pixel, overlay_pixel));
        }
    }
    DynamicImage::ImageRgba8(new_image)
}

/// Blend two pixels together based on their alpha values (transparency).
///
/// This function blends a pixel from the base image with a pixel from the overlay image. The
/// blending is done by considering the alpha (transparency) of the overlay pixel, which determines
/// how much of the overlay pixel should be visible over the base pixel.
///
/// # Parameters
///
/// * `base: Rgba<u8>` - The base pixel.
/// * `overlay: Rgba<u8>` - The pixel of the image you want to overlay.
///
/// # Returns
///
/// * `Rgba<u8>` - The pixel's new alpha value.
fn blend_pixels(base: Rgba<u8>, overlay: Rgba<u8>) -> Rgba<u8> {
    let alpha_overlay: f32 = overlay[3] as f32 / 255.0;

    let func = |b, o| -> u8 {
        (((b as f32 * (1.0 - alpha_overlay)) + (o as f32 * alpha_overlay)) * 255.0) as u8
    };

    Rgba([
        func(base[0], overlay[0]),
        func(base[1], overlay[1]),
        func(base[2], overlay[2]),
        255 as u8,
    ])
}
