pub mod configuration;
mod network;

pub extern crate async_trait;
pub extern crate tokio;
pub extern crate tcp_handler;
#[cfg(feature = "serde")]
pub extern crate serde;

use async_trait::async_trait;
use tcp_handler::common::{AesCipher, StarterError};
use tokio::net::{TcpStream, ToSocketAddrs};

pub use network::{send, recv};
use crate::network::start_client;

#[async_trait]
pub trait Client {
    fn get_identifier(&self) -> &'static str;

    fn get_version(&self) -> &'static str;

    async fn connect<A: ToSocketAddrs + Send>(&self, addr: A) -> Result<(TcpStream, AesCipher), StarterError> {
        start_client(self, addr).await
    }
}
