use vnc::{PixelFormat, VncClient, VncConnector, VncError};
use tokio::{self, net::TcpStream};

pub async fn create_vnc_client(target_ip: String, psw: Option<String>) -> Result<VncClient, VncError> {
    let tcp: TcpStream = TcpStream::connect(target_ip).await?;
    let vnc: VncClient = VncConnector::new(tcp)
        .set_auth_method(async move { Ok(psw.unwrap())})
        .add_encoding(vnc::VncEncoding::Tight)
        .add_encoding(vnc::VncEncoding::Zrle)
        .add_encoding(vnc::VncEncoding::CopyRect)
        .add_encoding(vnc::VncEncoding::Raw)
        .allow_shared(true)
        .set_pixel_format(PixelFormat::bgra())
        .build()?
        .try_start()
        .await?
        .finish()?;

    Ok(vnc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name_of_val;


    #[tokio::test]
    async fn test_build_client() {
        let res = create_vnc_client("192.168.0.1".to_string(), Some("Hello".to_string()));
        assert!(type_name_of_val(&res).contains("VncClient"));
   }
}
