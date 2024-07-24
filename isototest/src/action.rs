// SPDX-FileCopyrightText: Christopher Hock <christopher-hock@suse.com>
// SPDX-LicenseIdentifier: GPL-2.0-or-later
//! # Action
//!
//! This module handles interactions between the VncClient and VncServer.
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
fn char_to_keycode(c: char) -> Option<u32> {
    let keycode = match c {
        '1' => Some(KeyCode::Key1),
        '2' => Some(KeyCode::Key2),
        '3' => Some(KeyCode::Key3),
        '4' => Some(KeyCode::Key4),
        '5' => Some(KeyCode::Key5),
        '6' => Some(KeyCode::Key6),
        '7' => Some(KeyCode::Key7),
        '8' => Some(KeyCode::Key8),
        '9' => Some(KeyCode::Key9),
        '0' => Some(KeyCode::Key0),
        'A' => Some(KeyCode::A),
        'B' => Some(KeyCode::B),
        'C' => Some(KeyCode::C),
        'D' => Some(KeyCode::D),
        'E' => Some(KeyCode::E),
        'F' => Some(KeyCode::F),
        'G' => Some(KeyCode::G),
        'H' => Some(KeyCode::H),
        'I' => Some(KeyCode::I),
        'J' => Some(KeyCode::J),
        'K' => Some(KeyCode::K),
        'L' => Some(KeyCode::L),
        'M' => Some(KeyCode::M),
        'N' => Some(KeyCode::N),
        'O' => Some(KeyCode::O),
        'P' => Some(KeyCode::P),
        'Q' => Some(KeyCode::Q),
        'R' => Some(KeyCode::R),
        'S' => Some(KeyCode::S),
        'T' => Some(KeyCode::T),
        'U' => Some(KeyCode::U),
        'V' => Some(KeyCode::V),
        'W' => Some(KeyCode::W),
        'X' => Some(KeyCode::X),
        'Y' => Some(KeyCode::Y),
        'Z' => Some(KeyCode::Z),
        'a' => Some(KeyCode::a),
        'b' => Some(KeyCode::b),
        'c' => Some(KeyCode::c),
        'd' => Some(KeyCode::d),
        'e' => Some(KeyCode::e),
        'f' => Some(KeyCode::f),
        'g' => Some(KeyCode::g),
        'h' => Some(KeyCode::h),
        'i' => Some(KeyCode::i),
        'j' => Some(KeyCode::j),
        'k' => Some(KeyCode::k),
        'l' => Some(KeyCode::l),
        'm' => Some(KeyCode::m),
        'n' => Some(KeyCode::n),
        'o' => Some(KeyCode::o),
        'p' => Some(KeyCode::p),
        'q' => Some(KeyCode::q),
        'r' => Some(KeyCode::r),
        's' => Some(KeyCode::s),
        't' => Some(KeyCode::t),
        'u' => Some(KeyCode::u),
        'v' => Some(KeyCode::v),
        'w' => Some(KeyCode::w),
        'x' => Some(KeyCode::x),
        'y' => Some(KeyCode::y),
        'z' => Some(KeyCode::z),
        ' ' => Some(KeyCode::SPACE),
        '!' => Some(KeyCode::ExcMrk),
        '@' => Some(KeyCode::At),
        '#' => Some(KeyCode::Pound),
        '$' => Some(KeyCode::Dollar),
        '%' => Some(KeyCode::Percent),
        '^' => Some(KeyCode::Caret),
        '&' => Some(KeyCode::And),
        '*' => Some(KeyCode::Ast),
        '(' => Some(KeyCode::LRBrace),
        ')' => Some(KeyCode::RRBrace),
        '-' => Some(KeyCode::Minus),
        '_' => Some(KeyCode::UScore),
        '=' => Some(KeyCode::Equals),
        '+' => Some(KeyCode::Plus),
        '[' => Some(KeyCode::LBracket),
        ']' => Some(KeyCode::RBracket),
        '{' => Some(KeyCode::LCrlBrace),
        '}' => Some(KeyCode::RCrlBrace),
        '\\' => Some(KeyCode::BckSlash),
        '|' => Some(KeyCode::Pipe),
        ';' => Some(KeyCode::SColon),
        ':' => Some(KeyCode::Colon),
        '\'' => Some(KeyCode::SQuote),
        '"' => Some(KeyCode::DblQuote),
        ',' => Some(KeyCode::Comma),
        '.' => Some(KeyCode::Period),
        '/' => Some(KeyCode::FwdSlash),
        '<' => Some(KeyCode::LThan),
        '>' => Some(KeyCode::GThan),
        '?' => Some(KeyCode::Question),
        _ => None,
    };

    keycode.map(|kc| kc as u32)
}
