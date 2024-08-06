//! Schedule and run tests for [openQA](https://open.qa).
//!
//! This crate's only responsibility is to schedule and run tests for the openQA project.
//! To this end it connects to the test environment on a remote worker machine (VM or bare metal) which has been prepared
//! by its two "sister-libraries" `isotoenv` and `ìsotomachine` via VNC and executes commands
//! specified by the openQA test to run.
//!
//! ## Example
//!
//! To use this crate, you need to create a `VncClient` instance, which will connect you to your
//! VNC server. This instance must be passed to any function which communicated with the VNC
//! server.
//!
//! ```no_run
//! use isototest::connection::{create_vnc_client, kill_client};
//! use isototest::action::keyboard::write_to_console;
//! use isototest::action::view::read_screen;
//! use tokio::{self};
//! use std::process::exit;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let addr = "127.0.0.1:5900";
//!     let psw = "password".to_string(); // Value irrelevant if the server does not use authentication.
//!     let mut client = match create_vnc_client(addr.to_string(), Some(psw.clone())).await {
//!         Ok(client) => {
//!             println!("Client created. Handshake successful.");
//!             client
//!         },
//!         Err(e) => {
//!             eprintln!("[Error] {:?}", e);
//!             exit(1)
//!         }
//!     };
//!
//!     // Request screenshot from the remote machine, save the resolution as the client can not
//!     // request it again as long as it does not change.
//!     let res;
//!     let mut resolution = match read_screen(&client, "screenshot.png", None, Duration::from_secs(1)).await {
//!         Ok(x) => {
//!             println!("Screenshot received!");
//!             res = x;
//!         }
//!         Err(e) => {
//!             eprintln!("{}", e);
//!             exit(1);
//!         }
//!     };
//!
//!     // Send a series of keypresses to the VNC server to type out the given text.
//!     // Can be used to execute commands on the Terminal.
//!     match write_to_console(&client, "Hello World!\n".to_string(), None).await {
//!         Ok(_) => {
//!             println!("Test text sent!");
//!         }
//!         Err(e) => {
//!             eprintln!("[error] {:?}", e);
//!             exit(1);
//!         }
//!     }
//!
//!     // Kill VNC connection and release resources.
//!     kill_client(client).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Optional Features
//!
//! * `default-logging` - Provides you with a sensible logger configuration using the `env_logger`
//! crate.

pub mod action;
pub mod connection;
pub mod logging;
pub(crate) mod types;

#[cfg(feature = "default-logging")]
pub fn init_logging() {
    logging::initialize_default_logging();
}
