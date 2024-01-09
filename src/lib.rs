pub mod configuration;
mod network;
pub mod mutable_cipher;
pub mod client_base;

pub extern crate async_trait;
pub extern crate tokio;
pub extern crate tcp_handler;
#[cfg(feature = "serde")]
pub extern crate serde;

use async_trait::async_trait;
use tcp_handler::common::{AesCipher, StarterError};
use tokio::net::{TcpStream, ToSocketAddrs};
use crate::network::start_client;

pub use network::{send, recv};

#[async_trait]
pub trait ClientFactory<C: From<(TcpStream, AesCipher)>> {
    fn get_identifier(&self) -> &'static str;

    fn get_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION", "You should define the version in the manifest or override this method.")
    }

    async fn connect<A: ToSocketAddrs + Send>(&self, addr: A) -> Result<C, StarterError> {
        start_client(self, addr).await.map(|c| c.into())
    }
}

pub async fn quickly_connect<A: ToSocketAddrs + Send, C: From<(TcpStream, AesCipher)>>(identifier: &'static str, addr: A) -> Result<C, StarterError> {
    struct TempClient(&'static str);
    impl ClientFactory<(TcpStream, AesCipher)> for TempClient {
        fn get_identifier(&self) -> &'static str {
            self.0
        }
    }
    let client = TempClient(identifier);
    client.connect(addr).await.map(|c| c.into())
}
