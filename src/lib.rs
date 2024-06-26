#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod config;
pub mod errors;

pub extern crate bytes;
pub extern crate tcp_handler;

/// The main macro provided by this crate.
///
/// # Example
/// ```rust,no_run
/// use tcp_client::define_client;
/// use tcp_client::errors::Result;
///
/// define_client!(pub CommonMyClient, MyClient, "MyTcpApplication");
///
/// impl MyClient {
///     // define your method here.
///     // example:
///     async fn my_method(&mut self) -> Result<()> {
///         self.check_func("my_method").await?;
///         // ...
///         Ok(())
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let mut client = MyClient::connect("127.0.0.1:1234").await.unwrap();
///     // use client.
///     // example:
///     client.my_method().await.unwrap();
/// }
/// ```
#[macro_export]
macro_rules! define_client {
    ($vis: vis $client: ident, $tcp_client: ident, $identifier: literal) => {
        define_client!(compress_encrypt, $vis $client, $tcp_client, $identifier);
    };
    (raw, $vis: vis $client: ident, $tcp_client: ident, $identifier: literal) => {
        define_client!(@@define raw, TcpClientHandlerRaw, $vis $client, $tcp_client, $identifier);
    };
    (compress, $vis: vis $client: ident, $tcp_client: ident, $identifier: literal) => {
        define_client!(@@define compress, TcpClientHandlerCompress, $vis $client, $tcp_client, $identifier);
    };
    (encrypt, $vis: vis $client: ident, $tcp_client: ident, $identifier: literal) => {
        define_client!(@@define encrypt, TcpClientHandlerEncrypt, $vis $client, $tcp_client, $identifier);
    };
    (compress_encrypt, $vis: vis $client: ident, $tcp_client: ident, $identifier: literal) => {
        define_client!(@@define compress_encrypt, TcpClientHandlerCompressEncrypt, $vis $client, $tcp_client, $identifier);
    };

    (@@define $protocol: ident, $inner: ident, $vis: vis $client: ident, $tcp_client: ident, $identifier: literal) => {
        #[derive(Debug)]
        $vis struct $client<R: ::tokio::io::AsyncRead + ::core::marker::Unpin, W: ::tokio::io::AsyncWrite + ::core::marker::Unpin> {
            identifier: &'static str,
            version: &'static str,
            inner: ::tcp_handler::streams::$protocol::$inner<R, W>,
        }
        #[allow(dead_code)]
        impl<R: ::tokio::io::AsyncRead + ::core::marker::Unpin, W: ::tokio::io::AsyncWrite + ::core::marker::Unpin> $client<R, W> {
            $vis async fn new(reader: R, writer: W) -> $crate::errors::Result<Self> {
                let identifier = $identifier;
                let version = env!("CARGO_PKG_VERSION");
                let future = ::tcp_handler::streams::$protocol::$inner::new(reader, writer, identifier, version);
                let timeout = $crate::config::get_connect_timeout();
                let inner = ::tokio::time::timeout(timeout, future).await
                    .map_err(|_| $crate::errors::Error::Timeout(true, timeout))??;
                Ok(Self { identifier, version, inner })
            }
        }
        #[allow(dead_code)]
        impl<R: ::tokio::io::AsyncRead + ::core::marker::Unpin, W: ::tokio::io::AsyncWrite + ::core::marker::Unpin> $client<R, W> {
            #[inline]
            $vis fn get_identifier(&self) -> &'static str {
                &self.identifier
            }

            #[inline]
            $vis fn get_version(&self) -> &'static str {
                &self.version
            }

            #[inline]
            $vis async fn send<B: ::bytes::Buf>(&mut self, message: &mut B) -> $crate::errors::Result<()> {
                self.inner.send(message).await.map_err(|e| e.into())
            }

            #[inline]
            $vis async fn recv(&mut self) -> $crate::errors::Result<::bytes::BytesMut> {
                let future = self.inner.recv();
                let timeout = $crate::config::get_receive_timeout();
                ::tokio::time::timeout(timeout, future).await
                    .map_err(|_| $crate::errors::Error::Timeout(false, timeout))?
                    .map_err(|e| e.into())
            }

            #[inline]
            $vis async fn send_recv<B: ::bytes::Buf>(&mut self, message: &mut B) -> $crate::errors::Result<::bytes::BytesMut> {
                self.send(message).await?;
                self.recv().await
            }

            $vis async fn check_func(&mut self, func: &str) -> $crate::errors::Result<()> {
                use ::bytes::{Buf, BufMut};
                use ::variable_len_reader::{VariableReader, VariableWriter};
                let mut writer = ::bytes::BytesMut::new().writer();
                writer.write_string(func)?;
                let mut reader = self.send_recv(&mut writer.into_inner()).await?.reader();
                if reader.read_bool()? {
                    Ok(())
                } else {
                    Err($crate::errors::Error::ServerDenied)
                }
            }
        }
        $vis type $tcp_client = $client<::tokio::io::BufReader<::tokio::net::tcp::OwnedReadHalf>, ::tokio::io::BufWriter<::tokio::net::tcp::OwnedWriteHalf>>;
        #[allow(dead_code)]
        impl $tcp_client {
            $vis async fn connect<A: ::tokio::net::ToSocketAddrs>(addr: A) -> $crate::errors::Result<Self> {
                let identifier = $identifier;
                let version = env!("CARGO_PKG_VERSION");
                let future = ::tcp_handler::streams::$protocol::$inner::connect(addr, identifier, version);
                let timeout = $crate::config::get_connect_timeout();
                let inner = ::tokio::time::timeout(timeout, future).await
                    .map_err(|_| $crate::errors::Error::Timeout(true, timeout))??;
                Ok(Self { identifier, version, inner })
            }
        }
    };
}

#[cfg(test)]
mod tests {
    define_client!(DefaultClient, TcpDefaultClient, "DefaultClient");

    define_client!(raw, RawClient, TcpRawClient, "RawClient");
    define_client!(compress, CompressClient, TcpCompressClient, "CompressClient");
    define_client!(encrypt, EncryptClient, TcpEncryptClient, "EncryptClient");
    define_client!(compress_encrypt, CompressEncryptClient, TcpCompressEncryptClient, "CompressEncryptClient");
}
