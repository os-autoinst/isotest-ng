// SPDX-FileCopyrightText: Christopher Hock <christopher-hock@suse.com>
// SPDX-LicenseIdentifier: GPL-2.0-or-later
//! # Action
//!
//! This module handles interactions between the VncClient and VncServer.
extern crate proc_macro;
use std::{thread::sleep, time::Duration};

use vnc::{client::VncClient, ClientKeyEvent, VncError, X11Event};

use crate::types::KeyCode;

/// Sleep.
/// Needed to time requests in accordance with the server's framerate to not overwhelm it with
/// requests.
macro_rules! wait_for_frame {
    ($framerate:expr) => {
        match framerate_to_nanos($framerate) {
            Ok(nanos) => {
                sleep(nanos);
                Ok(())
            }
            Err(e) => Err(e),
        }
    };
}

/// Type of key press.
///
/// # Members
///
/// * `Press` - To signal a press and hold of the given key.
/// * `Release` - To signal the release of a given key.
/// * `Tap` - To signal a tap (press & release) of the given key.
enum KeyEventType {
    Press,
    Release,
    Tap,
}

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
pub async fn write_to_console(
    client: &VncClient,
    text: String,
    framerate: Option<f64>,
) -> Result<(), VncError> {
    // Translate each character to a keycode
    let mut keycode: u32;

    for ch in text.chars() {
        // Translate each character to its corresponding keycode.
        keycode = match char_to_keycode(ch) {
            Ok(code) => code,
            Err(e) => return Err(e),
        };

        // HACK: Check for characters requiring a modifier.
        // TODO/FIXME:
        //       - Find better solution for the mod checking.
        //       - Unify this if-block if it is kept.
        //       - Find a better way than unwrapping in a function call.
        if add_shift(ch).is_some() {
            // Press modifier
            press_button(
                client,
                add_shift(ch).unwrap(),
                KeyEventType::Press,
                framerate,
            )
            .await?;

            // Tap button for character
            press_button(client, keycode, KeyEventType::Tap, framerate).await?;

            // Release modifier
            press_button(
                client,
                add_shift(ch).unwrap(),
                KeyEventType::Release,
                framerate,
            )
            .await?;

            // TODO: make this go away.
            continue;
        }
        if add_ctrl(ch).is_some() {
            // Press modifier
            press_button(
                client,
                add_ctrl(ch).unwrap(),
                KeyEventType::Press,
                framerate,
            )
            .await?;

            // Tap key
            press_button(client, keycode, KeyEventType::Tap, framerate).await?;

            // Release modifier
            press_button(
                client,
                add_ctrl(ch).unwrap(),
                KeyEventType::Release,
                framerate,
            )
            .await?;

            // TODO: make this go away.
            continue;
        }
        press_button(client, keycode, KeyEventType::Tap, framerate).await?;
    }
    Ok(())
}

#[allow(unused)]
/// Receive a screenshot of the remote machine.
pub async fn read_screen(client: &VncClient) -> Result<(), VncError> {
    todo!("Not implemented yet!")
}

/// Encapsulate the `client.input()` function calls to avoid repitition.
///
/// Press and release a button or release it manually, if it is pressed.
///
/// # Parameters
///
/// * client: `&VncClient` - Reference to the client used for communication.
/// * keycode: `u32` - The keycode of the button to press.
/// * evtype: `KeyEventType` - Select whether the key is tapped, held or released.
/// * framerate: `Option<f64>` - The framerate of the target device. (default: 30)
///
/// # Returns
///
/// * `Ok(())` - If the keypress has been sent correctly.
/// * `Err(VncError)` - If an error occured during communication.
async fn press_button(
    client: &VncClient,
    keycode: u32,
    evtype: KeyEventType,
    framerate: Option<f64>,
) -> Result<(), VncError> {
    match evtype {
        KeyEventType::Press => {
            let kevent: ClientKeyEvent = ClientKeyEvent {
                keycode,
                down: true,
            };
            let xevent: X11Event = X11Event::KeyEvent(kevent);

            client.input(xevent).await?;
            wait_for_frame!(framerate)?;
        }
        KeyEventType::Release => {
            let kevent: ClientKeyEvent = ClientKeyEvent {
                keycode,
                down: false,
            };
            let xevent: X11Event = X11Event::KeyEvent(kevent);

            client.input(xevent).await?;
            wait_for_frame!(framerate)?;
        }
        KeyEventType::Tap => {
            let mut kevent: ClientKeyEvent = ClientKeyEvent {
                keycode,
                down: true,
            };
            let mut xevent: X11Event = X11Event::KeyEvent(kevent.clone());

            client.input(xevent).await?;
            wait_for_frame!(framerate)?;

            kevent.down = false;
            xevent = X11Event::KeyEvent(kevent);

            client.input(xevent).await?;
            wait_for_frame!(framerate)?;
        }
    }
    Ok(())
}

// HACK
/// Check if the fiven char requires SHIFT to be pressed.
///
/// # Parameters
///
/// * c: `char` - The character to be checked.
///
/// # Returns
///
/// * `Some(u32)` - Returns a `ClientKeyEvent` using the `LSHIFT` KeyCode.
/// * `None` - if the char does not require it.
fn add_shift(c: char) -> Option<u32> {
    const MOD_CHARS: &[char] = &[
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|', ':', '"', '<',
        '>', '?', '~',
    ];

    if c.is_ascii_uppercase() || MOD_CHARS.contains(&c) {
        return Some(KeyCode::LSHIFT as u32);
    }
    None
}

// HACK
/// Check if the given char requires CTRL or similar.
///
/// # Parameters
///
/// * c: `char` - The character to check.
///
/// # Returns
///
/// * `Some(ClientKeyEvent)` - Returns a LCTRL `KeyEvent` if the character requires it.
/// * `None` - If the char does not require it.
fn add_ctrl(c: char) -> Option<u32> {
    let ascii_value = c as u8;
    if ascii_value <= 0x1F || ascii_value == 0x7F {
        return Some(KeyCode::LCTRL as u32);
    }
    None
}

/// Calculate the nanoseconds in between signals.
///
/// Required as not to overflow the server's input buffer.
///
/// # Parameters
///
/// * rate: `i32` - The target framerate of the device. (30 by default)
///
/// Returns:
///
/// * `Ok(Duration)` - New `Duration` to time signals to the VNC server.
/// * `Err(VncError)` - A generic `VncError` to indicate wrong use of the function.
fn framerate_to_nanos(rate: Option<f64>) -> Result<Duration, VncError> {
    match rate {
        None => Ok(Duration::new(0, 20000000)),
        Some(r) => {
            if r <= 0.0 {
                return Err(VncError::General(
                    "[error] Framerate cannot be equal or less than zero!".to_string(),
                ));
            } // Will cut-off the floating point bits in the end.
            let secs_per_frame = (1.0 / rate.unwrap()) as u64;
            Ok(Duration::from_secs(secs_per_frame * 1000000000))
        }
    }
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
