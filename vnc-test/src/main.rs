use tokio::{self, net::TcpStream};
use isototest::create_vnc_client;

#[tokio::main]
async fn main() {
    match create_vnc_client("qamaster.qe.nue2.suse.org:14889".to_string(), Some(String::new())).await {
        Ok(client) => {
            println!("All ok!");
        },
        Err(e) => {
            panic!("{}", e);
        }
    };
}
