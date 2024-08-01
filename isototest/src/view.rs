use image::{GenericImage, ImageFormat, Rgba};
use std::{
    path::Path,
    time::{Duration, Instant},
};

use image::ImageBuffer;

use image::DynamicImage::ImageRgba8;
use vnc::{Rect, VncClient, VncError, VncEvent, X11Event};

/// Receive a screenshot of the remote machine.
pub async fn read_screen(
    client: &VncClient,
    file_path: &str,
    resolution: Option<(u32, u32)>,
    timeout: Duration,
) -> Result<(u32, u32), VncError> {
    // Request screen update
    client.input(X11Event::Refresh).await?;

    let mut img_parts: Vec<(Rect, Vec<u8>)> = Vec::new();
    let mut width;
    let mut height;
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

    let idle_timer = Instant::now();

    loop {
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
                    break;
                }
                x => {
                    println!("{:?}", x);
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

    // Reconstruct image.
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
    ImageRgba8(image)
        .save_with_format(path, ImageFormat::Png)
        .unwrap();

    println!("Screenshot saved to {}", file_path);
    Ok((width.unwrap(), height.unwrap()))
}
