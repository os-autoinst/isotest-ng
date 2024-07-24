// SPDX-FileCopyrightText: Christopher Hock <christopher-hock@suse.com>
// SPDX-LicenseIdentifier: GPL-2.0-or-later
//! # Action
//!
//! This module handles interactions between the VncClient and VncServer.
use std::{thread::sleep, time::Duration};

use vnc::{client::VncClient, ClientKeyEvent, VncError, X11Event};

use crate::types::KeyCode;

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
    let mut keycode: Result<u32, VncError>;
    for ch in text.chars() {
        // Translate each character to its corresponding keycode.
        keycode = char_to_keycode(ch);

        // Return error if key is not supported.
        // TODO: This may be removed, as soon as special keys are implemented.
        // Setup press events.
        let mut keyevent: ClientKeyEvent = ClientKeyEvent {
            keycode: match keycode {
                Ok(code) => code,
                Err(e) => return Err(e),
            },
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
        sleep(Duration::new(1, 0));
    }
    Ok(())
}

/// Receive a screenshot of the remote machine.
pub async fn read_screen(client: &VncClient) -> Result<(), VncError> {
    todo!("Not implemented yet!")
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
fn char_to_keycode(c: char) -> Result<u32, VncError> {
    let keycode = match c {
        '2' => Ok(KeyCode::Key2),
        '1' => Ok(KeyCode::Key1),
        '3' => Ok(KeyCode::Key3),
        '4' => Ok(KeyCode::Key4),
        '5' => Ok(KeyCode::Key5),
        '6' => Ok(KeyCode::Key6),
        '7' => Ok(KeyCode::Key7),
        '8' => Ok(KeyCode::Key8),
        '9' => Ok(KeyCode::Key9),
        '0' => Ok(KeyCode::Key0),
        'A' => Ok(KeyCode::A),
        'B' => Ok(KeyCode::B),
        'C' => Ok(KeyCode::C),
        'D' => Ok(KeyCode::D),
        'E' => Ok(KeyCode::E),
        'F' => Ok(KeyCode::F),
        'G' => Ok(KeyCode::G),
        'H' => Ok(KeyCode::H),
        'I' => Ok(KeyCode::I),
        'J' => Ok(KeyCode::J),
        'K' => Ok(KeyCode::K),
        'L' => Ok(KeyCode::L),
        'M' => Ok(KeyCode::M),
        'N' => Ok(KeyCode::N),
        'O' => Ok(KeyCode::O),
        'P' => Ok(KeyCode::P),
        'Q' => Ok(KeyCode::Q),
        'R' => Ok(KeyCode::R),
        'S' => Ok(KeyCode::S),
        'T' => Ok(KeyCode::T),
        'U' => Ok(KeyCode::U),
        'V' => Ok(KeyCode::V),
        'W' => Ok(KeyCode::W),
        'X' => Ok(KeyCode::X),
        'Y' => Ok(KeyCode::Y),
        'Z' => Ok(KeyCode::Z),
        'a' => Ok(KeyCode::a),
        'b' => Ok(KeyCode::b),
        'c' => Ok(KeyCode::c),
        'd' => Ok(KeyCode::d),
        'e' => Ok(KeyCode::e),
        'f' => Ok(KeyCode::f),
        'g' => Ok(KeyCode::g),
        'h' => Ok(KeyCode::h),
        'i' => Ok(KeyCode::i),
        'j' => Ok(KeyCode::j),
        'k' => Ok(KeyCode::k),
        'l' => Ok(KeyCode::l),
        'm' => Ok(KeyCode::m),
        'n' => Ok(KeyCode::n),
        'o' => Ok(KeyCode::o),
        'p' => Ok(KeyCode::p),
        'q' => Ok(KeyCode::q),
        'r' => Ok(KeyCode::r),
        's' => Ok(KeyCode::s),
        't' => Ok(KeyCode::t),
        'u' => Ok(KeyCode::u),
        'v' => Ok(KeyCode::v),
        'w' => Ok(KeyCode::w),
        'x' => Ok(KeyCode::x),
        'y' => Ok(KeyCode::y),
        'z' => Ok(KeyCode::z),
        ' ' => Ok(KeyCode::SPACE),
        '!' => Ok(KeyCode::ExcMrk),
        '@' => Ok(KeyCode::At),
        '#' => Ok(KeyCode::Pound),
        '$' => Ok(KeyCode::Dollar),
        '%' => Ok(KeyCode::Percent),
        '^' => Ok(KeyCode::Caret),
        '&' => Ok(KeyCode::And),
        '*' => Ok(KeyCode::Ast),
        '(' => Ok(KeyCode::LRBrace),
        ')' => Ok(KeyCode::RRBrace),
        '-' => Ok(KeyCode::Minus),
        '_' => Ok(KeyCode::UScore),
        '=' => Ok(KeyCode::Equals),
        '+' => Ok(KeyCode::Plus),
        '[' => Ok(KeyCode::LBracket),
        ']' => Ok(KeyCode::RBracket),
        '{' => Ok(KeyCode::LCrlBrace),
        '}' => Ok(KeyCode::RCrlBrace),
        '\\' => Ok(KeyCode::BckSlash),
        '|' => Ok(KeyCode::Pipe),
        ';' => Ok(KeyCode::SColon),
        ':' => Ok(KeyCode::Colon),
        '\'' => Ok(KeyCode::Apo),
        '"' => Ok(KeyCode::DblQuote),
        ',' => Ok(KeyCode::Comma),
        '.' => Ok(KeyCode::Period),
        '/' => Ok(KeyCode::FwdSlash),
        '<' => Ok(KeyCode::LThan),
        '>' => Ok(KeyCode::GThan),
        '?' => Ok(KeyCode::Question),
        '\n' => Ok(KeyCode::LineFeed),
        '`' => Ok(KeyCode::GraveAcc),
        _ => {
            return Err(VncError::General(format!(
                "Unable to identify ASCII code for character '{}'",
                c
            )))
        }
    };

    keycode.map(|kc| kc as u32)
}
