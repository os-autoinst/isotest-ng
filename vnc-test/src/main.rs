use isototest::action::write_to_console;
use isototest::connection::create_vnc_client;
use nix::sys::socket::{self, sockaddr_in, AddressFamily, SockType};
use std::process::exit;
use std::process::{Command, Stdio};
use std::ptr::null_mut;
use tokio::{
    self,
    net::{TcpListener, TcpStream},
};
use vnc::VncError;

// pub struct VncServer {
//     srv: Option<TcpListener>,
// }

// impl VncServer {
//     pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         let addr = Self::get_dynamic_port_address()?;
//         self.srv = Some(TcpListener::bind(addr).await?);
//         Ok(())
//     }

//     pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
//         if let Some(srv) = &self.srv {
//             drop(srv);
//         }
//         Ok(())
//     }

//     pub async fn spawn_vnc_server(&self) {
//         if let Some(srv) = &self.srv {
//             let mut socket = srv.accept().await.unwrap();
//             tokio::spawn(async move {
//                 let output = Command::new("vncserver")
//                     .stdin(Stdio::piped())
//                     .stdout(Stdio::piped())
//                     .stderr(Stdio::piped())
//                     .spawn()
//                     .unwrap();

//                 let _ = output.wait_with_output().unwrap();
//             });
//         }
//     }

//     fn get_dynamic_port_address() -> Result<String, Box<dyn std::error::Error>> {
//         let sock = unsafe { socket::socket(AddressFamily::AF_INET, SockType::SOCK_STREAM, 0)? };
//         let mut sin = sockaddr_in {
//             sin_family: AddressFamily::AF_INET as u16,
//             sin_port: 0,
//             sin_addr: std::net::Ipv4Addr::LOCALHOST.into(),
//             sin_zero: [0; 8],
//         };

//         let res =
//             unsafe { socket::getaddrinfo(null_mut(), b"vnc\0".as_ptr(), null_mut(), null_mut()) };
//         let addr = unsafe { res.get(0)?.addr };
//         let port = ((addr as *const sockaddr_in).cast::<u16>() as usize) % 65536;

//         Ok(format!("127.0.0.1:{}", port))
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut vnc_server = VncServer { srv: None };

    // println!("Starting...");

    // vnc_server.start().await?;
    // println!("Spawning connection...");
    // vnc_server.spawn_vnc_server().await;

    // println!("Creating client...");

    // println!("Getting port...");
    // let addr = vnc_server.srv.as_ref().unwrap().local_addr()?;
    let addr = "";
    let psw = "password".to_string();
    let client = match create_vnc_client(addr.to_string(), Some(psw)).await {
        Ok(client) => {
            println!("Client created. Handshake successful.");
            client
        }
        Err(e) => {
            eprintln!("[Error] {:?}.", e);
            exit(1);
        }
    };

    match write_to_console(&client, include_str!("test.txt").to_string(), None).await {
        Ok(_) => {
            println!("Test text sent!");
        }
        Err(e) => {
            eprintln!("[error] {:?}.", e);
            exit(1);
        }
    }

    Ok(())
}
