pub mod configuration;
mod network;

use async_trait::async_trait;
use tokio::net::{TcpStream, ToSocketAddrs};

#[async_trait]
pub trait Client {
    fn get_identifier(&self) -> &'static str;

    fn get_version(&self) -> &'static str;

    async fn connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<TcpStream> {
        let stream = TcpStream::connect(addr).await?;
    }
}
