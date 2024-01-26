#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod config;
pub mod network;
pub mod mutable_cipher;
pub mod client_base;

pub extern crate tokio;
pub extern crate tcp_handler;

use async_trait::async_trait;
use tcp_handler::common::AesCipher;
use tokio::net::{TcpStream, ToSocketAddrs};
use crate::network::{NetworkError, start_client};

/// The client factory to create clients.
/// # Example
/// ```rust,no_run
/// use tcp_client::client_base::ClientBase;
/// use tcp_client::client_factory;
/// use tcp_client::ClientFactory;
/// use tcp_client::network::NetworkError;
///
/// client_factory!(MyClientFactory, MyClient, "MyTcpApplication");
///
/// impl MyClient {
///     // define your method here.
///     // example:
///     async fn my_method(&mut self) -> Result<(), NetworkError> {
///         self.check_func("my_method").await?;
///         // ...
///         Ok(())
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let mut client = MyClientFactory.connect("127.0.0.1:1234").await.unwrap();
///     // use client.
///     // example:
///     client.my_method().await.unwrap();
/// }
/// ```
#[async_trait]
pub trait ClientFactory<C: From<(TcpStream, AesCipher)>> {
    /// Get the identifier of your application.
    /// # Note
    /// This should be a const.
    fn get_identifier(&self) -> &'static str;

    /// Get the version of your application.
    /// # Note
    /// This should be a const.
    /// # Example
    /// ```rust,ignore
    /// env!("CARGO_PKG_VERSION")
    /// ```
    fn get_version(&self) -> &'static str;

    /// Build a new client.
    async fn connect<A: ToSocketAddrs + Send>(&self, addr: A) -> Result<C, NetworkError> {
        start_client(self, addr).await.map(|c| c.into())
    }
}

/// Conveniently define a client factory.
/// # Example
/// ```rust,ignore
/// use tcp_client::client_factory;
///
/// client_factory!(MyClientFactory, MyClient, "MyTcpApplication");
/// ```
#[macro_export]
macro_rules! client_factory {
    ($factory_vis: vis $factory: ident, $client_vis: vis $client: ident, $identifier: literal) => {
        $factory_vis struct $factory;
        #[derive(Debug)]
        $client_vis struct $client {
            receiver: $crate::tokio::io::BufReader<$crate::tokio::net::tcp::OwnedReadHalf>,
            sender: $crate::tokio::io::BufWriter<$crate::tokio::net::tcp::OwnedWriteHalf>,
            cipher: $crate::mutable_cipher::MutableCipher,
        }

        impl $crate::ClientFactory<$client> for $factory {
            fn get_identifier(&self) -> &'static str {
                $identifier
            }

            fn get_version(&self) -> &'static str {
                env!("CARGO_PKG_VERSION")
            }
        }
        impl From<($crate::tokio::net::TcpStream, $crate::tcp_handler::common::AesCipher)> for $client {
            fn from(value: ($crate::tokio::net::TcpStream, $crate::tcp_handler::common::AesCipher)) -> Self {
                let (receiver, sender)= value.0.into_split();
                Self {
                    receiver: $crate::tokio::io::BufReader::new(receiver),
                    sender: $crate::tokio::io::BufWriter::new(sender),
                    cipher: $crate::mutable_cipher::MutableCipher::new(value.1),
                }
            }
        }
        impl $crate::client_base::ClientBase<$crate::tokio::io::BufReader<$crate::tokio::net::tcp::OwnedReadHalf>, $crate::tokio::io::BufWriter<$crate::tokio::net::tcp::OwnedWriteHalf>> for $client {
            fn get_receiver<'a>(&'a mut self) -> (&'a mut $crate::tokio::io::BufReader<$crate::tokio::net::tcp::OwnedReadHalf>, &$crate::mutable_cipher::MutableCipher) {
                (&mut self.receiver, &self.cipher)
            }

            fn get_sender<'a>(&'a mut self) -> (&'a mut $crate::tokio::io::BufWriter<$crate::tokio::net::tcp::OwnedWriteHalf>, &$crate::mutable_cipher::MutableCipher) {
                (&mut self.sender, &self.cipher)
            }
        }
    };
}

/// A shortcut for [`ClientFactory`].
pub async fn quickly_connect<A: ToSocketAddrs + Send, C: From<(TcpStream, AesCipher)>>(identifier: &'static str, version:&'static str, addr: A) -> Result<C, NetworkError> {
    struct TempClient(&'static str, &'static str);
    impl ClientFactory<(TcpStream, AesCipher)> for TempClient {
        fn get_identifier(&self) -> &'static str {
            self.0
        }
        fn get_version(&self) -> &'static str {
            self.1
        }
    }
    TempClient(identifier, version).connect(addr).await.map(|c| c.into())
}
