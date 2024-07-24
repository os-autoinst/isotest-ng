// SPDX-FileCopyrightText: Christopher Hock <christopher-hock@suse.com>
// SPDX-LicenseIdentifier: GPL-2.0-or-later
//! # Action
//!
//! This module handles interactions between the VncClient and VncServer.
use async_winit::event::VirtualKeyCode;
use vnc::{client::VncClient, ClientKeyEvent, VncError, X11Event};

/// Write given text to console
///
/// Uses `X11Event`s to send keypresses to the server. According to the [RFC](https://www.rfc-editor.org/rfc/rfc6143.html#section-7.5.4)
/// it does not matter whether the X-Window System is running or not.
///
/// # Parameters
///
/// * client: `&VncClient` - The client to be used for connections
/// * text: `String` - The text to write.
///
/// # Returns
///
/// * `Ok(())` - If the transaction has been successfully completed.
/// * `VncError` - If the transaction fails.
pub async fn write_to_console(client: &VncClient, text: &str) -> Result<(), VncError> {
    // Translate each character to a keycode
    let mut keycode: Option<u32>;
    for ch in text.chars() {
        // Translate each character to its corresponding keycode.
        keycode = char_to_keycode(ch);
        // Return error if key is not supported.
        // TODO: This may be removed, as soon as special keys are implemented.
        if keycode.is_none() {
            let e: VncError = VncError::General(format!("Unable to identify character '{}'!", ch));
            return Err(e);
        }
        // Setup press events.
        let mut keyevent: ClientKeyEvent = ClientKeyEvent {
            keycode: keycode.unwrap(),
            down: true,
        };
        let mut x_event: X11Event = X11Event::KeyEvent(keyevent);

        // Send individual keypresses.
        match client.input(x_event).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // NOTE: Is this really necessary?
        // Setup key release events.
        keyevent = ClientKeyEvent {
            keycode: keycode.unwrap(),
            down: false,
        };
        x_event = X11Event::KeyEvent(keyevent);

        // Send key releases.
        match client.input(x_event).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Assign a given character its corresponding `VirtualKeyCode`.
///
/// NOTE: This is only to be used in combination with sending text. Special characters and command
/// sequences are not yet implemented.
///
/// # Parameters
///
/// * c: `char` - The character to look up.
///
/// # Returns
///
/// * `Some(u32)` - The `u32` value of the  `VirtualKeyCode` corresponding to the character.
/// * `None` - If the character is not supported.
fn char_to_keycode(c: char) -> Option<u32> {
    let keycode = match c {
        '1' => Some(VirtualKeyCode::Key1),
        '2' => Some(VirtualKeyCode::Key2),
        '3' => Some(VirtualKeyCode::Key3),
        '4' => Some(VirtualKeyCode::Key4),
        '5' => Some(VirtualKeyCode::Key5),
        '6' => Some(VirtualKeyCode::Key6),
        '7' => Some(VirtualKeyCode::Key7),
        '8' => Some(VirtualKeyCode::Key8),
        '9' => Some(VirtualKeyCode::Key9),
        '0' => Some(VirtualKeyCode::Key0),
        'a' | 'A' => Some(VirtualKeyCode::A),
        'b' | 'B' => Some(VirtualKeyCode::B),
        'c' | 'C' => Some(VirtualKeyCode::C),
        'd' | 'D' => Some(VirtualKeyCode::D),
        'e' | 'E' => Some(VirtualKeyCode::E),
        'f' | 'F' => Some(VirtualKeyCode::F),
        'g' | 'G' => Some(VirtualKeyCode::G),
        'h' | 'H' => Some(VirtualKeyCode::H),
        'i' | 'I' => Some(VirtualKeyCode::I),
        'j' | 'J' => Some(VirtualKeyCode::J),
        'k' | 'K' => Some(VirtualKeyCode::K),
        'l' | 'L' => Some(VirtualKeyCode::L),
        'm' | 'M' => Some(VirtualKeyCode::M),
        'n' | 'N' => Some(VirtualKeyCode::N),
        'o' | 'O' => Some(VirtualKeyCode::O),
        'p' | 'P' => Some(VirtualKeyCode::P),
        'q' | 'Q' => Some(VirtualKeyCode::Q),
        'r' | 'R' => Some(VirtualKeyCode::R),
        's' | 'S' => Some(VirtualKeyCode::S),
        't' | 'T' => Some(VirtualKeyCode::T),
        'u' | 'U' => Some(VirtualKeyCode::U),
        'v' | 'V' => Some(VirtualKeyCode::V),
        'w' | 'W' => Some(VirtualKeyCode::W),
        'x' | 'X' => Some(VirtualKeyCode::X),
        'y' | 'Y' => Some(VirtualKeyCode::Y),
        'z' | 'Z' => Some(VirtualKeyCode::Z),
        ' ' => Some(VirtualKeyCode::Space),
        '!' => Some(VirtualKeyCode::Key1),
        '@' => Some(VirtualKeyCode::Key2),
        '#' => Some(VirtualKeyCode::Key3),
        '$' => Some(VirtualKeyCode::Key4),
        '%' => Some(VirtualKeyCode::Key5),
        '^' => Some(VirtualKeyCode::Caret),
        '&' => Some(VirtualKeyCode::Key7),
        '*' => Some(VirtualKeyCode::Key8),
        '(' => Some(VirtualKeyCode::Key9),
        ')' => Some(VirtualKeyCode::Key0),
        '-' => Some(VirtualKeyCode::Minus),
        '_' => Some(VirtualKeyCode::Underline),
        '=' => Some(VirtualKeyCode::Equals),
        '+' => Some(VirtualKeyCode::Plus),
        '[' => Some(VirtualKeyCode::LBracket),
        ']' => Some(VirtualKeyCode::RBracket),
        '{' => Some(VirtualKeyCode::LBracket),
        '}' => Some(VirtualKeyCode::RBracket),
        '\\' => Some(VirtualKeyCode::Backslash),
        '|' => Some(VirtualKeyCode::Backslash),
        ';' => Some(VirtualKeyCode::Semicolon),
        ':' => Some(VirtualKeyCode::Colon),
        '\'' => Some(VirtualKeyCode::Apostrophe),
        '"' => Some(VirtualKeyCode::Apostrophe),
        ',' => Some(VirtualKeyCode::Comma),
        '.' => Some(VirtualKeyCode::Period),
        '/' => Some(VirtualKeyCode::Slash),
        '<' => Some(VirtualKeyCode::Comma),
        '>' => Some(VirtualKeyCode::Period),
        '?' => Some(VirtualKeyCode::Slash),
        _ => None,
    };

    keycode.map(|kc| kc as u32)
}
