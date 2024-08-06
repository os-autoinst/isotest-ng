use image::{GenericImage, ImageFormat, Rgba};
use std::{
    path::Path,
    time::{Duration, Instant},
};

use image::ImageBuffer;

use image::DynamicImage::ImageRgba8;
use vnc::{Rect, VncClient, VncError, VncEvent, X11Event};

/// Receive a screenshot of the remote machine.
///
/// # Parameters
///
/// * client: `&VncClient` - The client instance used for connection.
/// * file_path: `&str` - A file path you want to save your screenshot under as a `str`.
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
    file_path: &str,
    resolution: Option<(u32, u32)>,
    timeout: Duration,
) -> Result<(u32, u32), VncError> {
    // Request screen update.
    client.input(X11Event::Refresh).await?;

    let mut img_parts: Vec<(Rect, Vec<u8>)> = Vec::new();
    let mut width: Option<u32>;
    let mut height: Option<u32>;

    // Try to detect screen resolution of the remote machine if it has not been passed.
    // **This will cause issues, if you try to use this functionality a second time.**
    match resolution {
        Some((x, y)) => {
            width = Some(x);
            height = Some(y);
        }
        None => match client.recv_event().await? {
            VncEvent::SetResolution(screen) => {
                println!("Screen resolution: {}x{}", screen.width, screen.height);
                width = Some(screen.width as u32);
                height = Some(screen.height as u32);

                client.input(X11Event::Refresh).await?;
            }
            _ => {
                return Err(VncError::General(
                    "[error] No resolution found!".to_string(),
                ))
            }
        },
    }

    let path: &Path = Path::new(file_path);
    let idle_timer: Instant = Instant::now();

    loop {
        // Poll new vnc events.
        match client.poll_event().await? {
            Some(x) => match x {
                VncEvent::SetResolution(screen) => {
                    println!("Screen resolution: {}x{}", screen.width, screen.height);
                    width = Some(screen.width as u32);
                    height = Some(screen.height as u32);

                    client.input(X11Event::Refresh).await?;
                }
                VncEvent::RawImage(rect, data) => {
                    img_parts.push((rect, data));
                }
                VncEvent::Error(e) => {
                    eprintln!("[error] {}", e);
                    return Err(VncError::General(e));
                }
                x => {
                    println!(
                        "[warning] Function 'read_screen' got unexpected event '{:?}'.",
                        x
                    );
                    break;
                }
            },
            None => {
                if idle_timer.elapsed() >= timeout {
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

    // Save image to file system in PNG format.
    // NOTE: If the image color encoding is changed here, you must also change it in connection.rs!
    ImageRgba8(image)
        .save_with_format(path, ImageFormat::Png)
        .unwrap();

    println!("Screenshot saved to {}", file_path);
    Ok((width.unwrap(), height.unwrap()))
}
